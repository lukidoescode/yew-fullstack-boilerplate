// https://github.com/yewstack/yew/tree/master/examples/pub_sub/src

use crate::model::Auth;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashSet;
use yew::agent::{Agent, AgentLink, Context, HandlerId};
use yew::services::{storage::Area, StorageService};

static AUTH_KEY: &str = "Auth";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Request {
    Login(Auth),
    Logout,
}

pub struct AuthEventBus {
    link: AgentLink<AuthEventBus>,
    subscribers: HashSet<HandlerId>,
    auth: Option<Auth>,
    storage: Option<StorageService>,
}

impl Agent for AuthEventBus {
    type Reach = Context;
    type Message = ();
    type Input = Request;
    type Output = Option<Auth>;

    fn create(link: AgentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Session).ok();
        let auth: Option<Auth> = {
            if let Some(storage) = &storage {
                let auth_str: Option<String> = {
                    if let Ok(auth_str) = storage.restore(AUTH_KEY) {
                        Some(auth_str)
                    } else {
                        None
                    }
                };
                match auth_str {
                    Some(auth_str) => serde_json::from_str(auth_str.as_str()).ok(),
                    None => None,
                }
            } else {
                None
            }
        };
        // let auth = storage.restore::<Auth>(AUTH).ok();
        AuthEventBus {
            link,
            subscribers: HashSet::new(),
            auth,
            storage,
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        match msg {
            Request::Login(auth) => {
                self.auth = Some(auth.clone());
            }
            Request::Logout => {
                self.auth = None;
            }
        }
        if let Some(storage) = &mut self.storage {
            match &self.auth {
                Some(auth) => {
                    match serde_json::to_string(auth).ok() {
                        Some(auth_str) => {
                            storage.store(AUTH_KEY, Ok(auth_str));
                        }
                        None => storage.remove(AUTH_KEY),
                    };
                }
                None => storage.remove(AUTH_KEY),
            };
        }
        for sub in self.subscribers.iter() {
            self.link.respond(*sub, self.auth.clone());
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.link.respond(id, self.auth.clone());
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
