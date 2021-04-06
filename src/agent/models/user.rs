use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct User {
    pub uid: u32,
    pub name: String,
}

pub struct Users {
    items: Vec<User>
}

impl Users {
    /// Create users
    pub fn new() -> Self {
        Users {
            items: vec![]
        }
    }

    /// Add user to users
    ///
    /// * `user` - a user
    pub fn add(&mut self, user: User) {
        self.items.push(user)
    }

    /// Get user by uid
    ///
    /// * `uid` - uid of user
    pub fn get(&self, uid: u32) -> Option<&User> {
        for user in &self.items {
            if user.uid == uid {
                return Some(&user)
            }
        }
        None
    }
}