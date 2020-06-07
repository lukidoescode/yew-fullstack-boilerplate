use serde::{Deserialize, Serialize};

mod auth;

pub use auth::Auth;
pub use auth::Login;
pub use auth::Signup;
pub use auth::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerResponse<T> {
    pub message: String,
    pub data: T,
}
