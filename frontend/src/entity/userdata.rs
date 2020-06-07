use crate::{entity::Username, storage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Userdata {
    pub username: Username,
    pub auth_token: String,
}

impl Userdata {
    pub const fn username(&self) -> &Username {
        &self.profile.username
    }

    pub fn store(&self) {
        storage::store_userdata(self);
    }
}
