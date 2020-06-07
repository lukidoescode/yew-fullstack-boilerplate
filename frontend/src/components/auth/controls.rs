use crate::agents::auth::{AuthEventBus, Request as AuthEventBusRequest};
use crate::model::Auth;
use anyhow::Error;
use css_in_rust::Style;
use yew::{
    agent::{Dispatched, Dispatcher},
    format::Nothing,
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Callback, Component, ComponentLink, Html, MouseEvent, Properties, ShouldRender,
};

pub struct Controls {
    link: ComponentLink<Self>,
    props: Props,
    style: Style,
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    auth_event_bus: Dispatcher<AuthEventBus>,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    pub auth: Auth,
}

pub enum Msg {
    Logout,
    Response,
}

impl Component for Controls {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create("auth-controls", include_str!("controls.scss"))
            .expect("An error occured while creating the style.");
        Self {
            link,
            props,
            style,
            fetch_service: FetchService::new(),
            fetch_task: None,
            auth_event_bus: AuthEventBus::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Logout => {
                let req = Request::post("/api/auth/logout")
                    .header("Authentication", format!("bearer {}", self.props.auth.jwt))
                    .body(Nothing)
                    .expect("Failed to build login request.");
                let fetch_task = self
                    .fetch_service
                    .fetch(
                        req,
                        self.link
                            .callback(|_response: Response<Result<String, Error>>| Msg::Response),
                    )
                    .unwrap();
                self.fetch_task = Some(fetch_task);
            }
            Msg::Response => {
                self.fetch_task = None;
                self.auth_event_bus.send(AuthEventBusRequest::Logout);
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let onclick_logout: Callback<MouseEvent> = self.link.callback(|ev: MouseEvent| {
            ev.prevent_default();
            Msg::Logout
        });
        html! {
            <div class=self.style.clone()>
                <span class="username">{ format!("Hi, {}!", &self.props.auth.user.username) }</span>
                <a onclick=onclick_logout>{ "Logout" }</a>
            </div>
        }
    }
}
