use crate::agents::auth::{AuthEventBus, Request as AuthEventBusRequest};
use crate::model::{Auth as AuthModel, Login as Model, ServerResponse, User as UserModel};
use anyhow::Error;
use css_in_rust::Style;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::{
    agent::{Dispatched, Dispatcher},
    format::Json,
    html,
    services::{
        fetch::{FetchTask, Request, Response, StatusCode},
        FetchService,
    },
    virtual_dom::Classes,
    Component, ComponentLink, Event, Html, InputData, Properties, ShouldRender,
};

pub struct Login {
    link: ComponentLink<Self>,
    props: Props,
    id: String,
    style: Style,
    form_data: Model,
    username_error: Option<String>,
    password_error: Option<String>,
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    auth_event_bus: Dispatcher<AuthEventBus>,
}

#[derive(Properties, Clone, Serialize, Deserialize, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
}

pub enum LoginFailure {
    BadUsername,
    BadPassword,
    Other,
}

pub enum Msg {
    UpdateEmailOrUsername(String),
    UpdatePassword(String),
    Request,
    Response(Result<String, LoginFailure>),
}

impl Component for Login {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create("login", include_str!("login.scss"))
            .expect("An error occured while creating the style.");
        Self {
            link,
            props,
            id: Uuid::new_v4().to_simple().to_string(),
            style,
            form_data: Model::default(),
            username_error: None,
            password_error: None,
            fetch_service: FetchService::new(),
            fetch_task: None,
            auth_event_bus: AuthEventBus::dispatcher(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateEmailOrUsername(email_or_username) => {
                if email_or_username.is_empty() {
                    self.username_error = Some("CANNOT_BE_EMPTY".to_string());
                } else {
                    self.username_error = None;
                }
                self.form_data.email_or_username = email_or_username;
            }
            Msg::UpdatePassword(password) => {
                if password.is_empty() {
                    self.password_error = Some("CANNOT_BE_EMPTY".to_string());
                } else {
                    self.password_error = None;
                }
                self.form_data.password = password;
            }
            Msg::Request => {
                let req = Request::post("/api/auth/login")
                    .header("Content-Type", "application/json")
                    .body(Json(&self.form_data))
                    .expect("Failed to build login request.");
                let fetch_task = self
                    .fetch_service
                    .fetch(
                        req,
                        self.link
                            .callback(|response: Response<Result<String, Error>>| {
                                match (&response).status() {
                                    StatusCode::OK => {
                                        // Msg::Response(Ok("User has been registered".to_string()))
                                        let empty = &"".to_string();
                                        let body = response.body().as_ref().unwrap_or(empty);
                                        Msg::Response(Ok(body.clone()))
                                    }
                                    StatusCode::NOT_FOUND => {
                                        Msg::Response(Err(LoginFailure::BadUsername))
                                    }
                                    StatusCode::BAD_REQUEST => {
                                        Msg::Response(Err(LoginFailure::BadPassword))
                                    }
                                    _ => Msg::Response(Err(LoginFailure::Other)),
                                }
                            }),
                    )
                    .unwrap();
                self.fetch_task = Some(fetch_task);
            }
            Msg::Response(res) => {
                match res {
                    Ok(data) => {
                        let login_info: ServerResponse<LoginSuccess> =
                            serde_json::from_str(data.as_str()).unwrap();
                        self.auth_event_bus
                            .send(AuthEventBusRequest::Login(AuthModel {
                                jwt: login_info.data.token,
                                user: login_info.data.user,
                            }))
                    }
                    Err(fail) => match fail {
                        LoginFailure::BadUsername => {
                            self.username_error = Some("BAD_USERNAME".to_string());
                        }
                        LoginFailure::BadPassword => {
                            self.password_error = Some("BAD_PASSWORD".to_string());
                        }
                        LoginFailure::Other => {}
                    },
                }
                self.fetch_task = None;
            }
        }
        true
    }

    fn view(&self) -> Html {
        let onsubmit = self.link.callback(|ev: Event| {
            ev.prevent_default();
            Msg::Request
        });
        let oninput_email_or_username = self
            .link
            .callback(|ev: InputData| Msg::UpdateEmailOrUsername(ev.value));
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));
        let mut username_class = "";
        if self.username_error.is_some() {
            username_class = "invalid"
        }
        let mut password_class = "";
        if self.password_error.is_some() {
            password_class = "invalid"
        }
        html! {
            <div class=Classes::from(self.props.class.clone()).extend(self.style.clone())>
                <form id=&self.id class="login" onsubmit=onsubmit>
                    <label for=format!("{}-email-or-userame", self.id)>
                        {"Email or Username"}
                    </label>
                    <input
                        id=format!("{}-email-or-userame", self.id)
                        type="text"
                        class=username_class
                        value=&self.form_data.email_or_username
                        oninput=oninput_email_or_username
                        placeholder="Email or Username"
                        spellcheck="false" />
                    <label for=format!("{}-password", self.id)>
                        {"Password"}
                    </label>
                    <input
                        id=format!("{}-password", self.id)
                        type="password"
                        class=password_class
                        value=&self.form_data.password
                        oninput=oninput_password
                        placeholder="Password"
                        spellcheck="false" />
                    <button type="submit" disabled=self.fetch_task.is_some()>
                        { "Log in" }
                    </button>
                </form>
            </div>
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginSuccess {
    token: String,
    token_type: String,
    user: UserModel,
}
