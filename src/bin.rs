mod agent;

use tide::{Request, Response, StatusCode};
use tide::http::mime;
use tide::sse;
use broadcaster::BroadcastChannel;
use futures::prelude::*;
use serde;
use serde::{Deserialize, Serialize};

use agent::os::linux::Linux;
use agent::models::process::{Processes};
use std::collections::HashMap;
use config::{Config, File};

#[derive(Clone, Debug)]
struct State {
    broadcaster: BroadcastChannel<Processes>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut settings = Config::default();
    settings
        .merge(File::with_name("Settings")).unwrap();
    let conf =  settings.try_into::<HashMap<String, String>>()?;
    let url = match conf.get("url") {
        Some(u) => u.clone(),
        None => "".to_string()
    };
    let secret = match conf.get("secret") {
        Some(s) => s.clone(),
        None => "".to_string()
    };

    let broadcaster = BroadcastChannel::new();
    let mut app = tide::with_state(State { broadcaster });
    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        secret.as_bytes()
    ));
    app.at("/data").get(sse::endpoint(|req: Request<State>, sender| async move {
        let state = req.state().clone();
        while let Some(processes) = state.broadcaster.clone().next().await {
            sender.send("processes", serde_json::to_string(&processes)?, None).await?;
        }
        Ok(())
    }));
    app.at("/acquire_process_list").post(acquire_process_list);
    app.at("/processes").get(processes);
    app.at("/search").get(search);
    println!("App : {}", &url);
    app.listen(&url).await?;
    Ok(())
}

/// Load all processes
async fn acquire_process_list(mut req: Request<State>) -> tide::Result {
    let linux = Linux::new();
    let res: String = serde_json::to_string(&linux.processes)?;
    let processes_str:String;
    match req.session().get::<String>("processes") {
        Some(r) => processes_str = r,
        None => processes_str = "[]".to_string()
    }
    let mut old_processes = Processes::new();

    match serde_json::from_str::<Processes>(processes_str.as_str()) {
        Ok(p) => old_processes = p,
        Err(_) => {}
    };
    let mut difference = Processes::new();
    for i in linux.processes.items {
        if !old_processes.items.contains(&i) {
            difference.add(i);
        }
    }
    req.session_mut().insert("processes", res.clone())?;
    req.state().broadcaster.send(&difference).await?;

    Ok(Response::new(StatusCode::Ok))
}

/// Return processes with json format
async fn processes(req: Request<State>) -> tide::Result {
    let resp:String;
    match req.session().get::<String>("processes") {
        Some(r) => resp = r,
        None => resp = "[]".to_string()
    }
    let response = Response::builder(200)
        .body(resp)
        .content_type(mime::JSON)
        .build();

    Ok(response)
}

#[derive(Debug, Deserialize, Serialize)]
struct Search {
    pid: Option<u32>,
    username: Option<String>
}

/// Search processes by pid and username
async fn search(req: Request<State>) -> tide::Result {
    let mut response = Response::builder(200)
        .body("[]")
        .content_type(mime::JSON)
        .build();
    let search:Search = match req.query::<Search>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{:?}", e);
            return Ok(response)
        }
    };
    let username = search.username;
    let pid = search.pid;
    match req.session().get::<String>("processes") {
        Some(r) => {
            match serde_json::from_str::<Processes>(r.as_str()) {
                Ok(mut processes) => {
                    match pid {
                        Some(p) => {
                            processes = processes.get(p);
                        }
                        None => {}
                    }
                    match username {
                        Some(u) => {

                            processes = processes.get_by_username(u);
                        }
                        None => {}
                    }

                    response.set_body(serde_json::to_string(&processes)?);
                    Ok(response)
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                    return Ok(response)
                }
            }
        },
        None => Ok(response)
    }
}