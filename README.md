# PROCMAN
![](https://img.shields.io/badge/unsafe-forbidden-success.svg)

## Description
ProcMan is a basic application to simply manage linux processes in Rust.
Through the application you can display the process list and monitor when a new process arrives.

🔒 Implemented using 100% safe rust and works on all platforms supported by rust

## Usage
```bash
cargo run
```
The application was tested with RESTer Firefox plugin.

## Endpoints
Here are the different endpoints:
- [POST] /acquire_process_list : load processes
- [GET] /processes : return processes
- [GET] /search : search processes by pid or username
- [GET] /data : SSE new processes 
