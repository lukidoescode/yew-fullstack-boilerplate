mod home;
mod profile;

use yew_router::switch::Permissive;
use yew_router::Switch;
// use yew_router::matcher::MatcherToken;

pub use home::Home;
pub use profile::Profile;

/// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoutes {
    #[to = "/profile"]
    Profile,
    #[to = "/!"]
    Home,
    #[to = "/{*:any}"]
    NotFound(Permissive<String>),
}
