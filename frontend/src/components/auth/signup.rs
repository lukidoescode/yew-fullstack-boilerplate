use crate::model::Signup as Model;
use anyhow::Error;
use css_in_rust::Style;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yew::{
    format::Json,
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    virtual_dom::Classes,
    Component, ComponentLink, Event, Html, InputData, Properties, ShouldRender,
};

// TODO make this into a common model lib
#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDTO {
    pub email: String,
    pub username: String,
    pub password: String,
    pub login_session: String,
}

pub struct Signup {
    props: Props,
    link: ComponentLink<Self>,
    id: String,
    style: Style,
    form_data: Model,
    response_data: Option<Result<String, String>>,
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
}

pub enum Msg {
    UpdateEmail(String),
    UpdateUsername(String),
    UpdatePassword(String),
    UpdatePasswordRepeat(String),
    Request,
    Response(Result<String, String>),
}

impl Component for Signup {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create("signup", include_str!("signup.scss"))
            .expect("An error occured while creating the style.");
        Self {
            props,
            link,
            id: Uuid::new_v4().to_simple().to_string(),
            style,
            form_data: Model::default(),
            response_data: None,
            fetch_service: FetchService::new(),
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateEmail(email) => {
                self.form_data.email = email;
            }
            Msg::UpdateUsername(username) => {
                self.form_data.username = username;
            }
            Msg::UpdatePassword(password) => {
                self.form_data.password = password;
            }
            Msg::UpdatePasswordRepeat(password) => {
                self.form_data.password_repeat = password;
            }
            Msg::Request => {
                let request_data = UserDTO::from(self.form_data.clone());
                let req = Request::post("/api/auth/signup")
                    .header("Content-Type", "application/json")
                    .body(Json(&request_data))
                    .expect("Failed to build signup request.");
                let fetch_task = self
                    .fetch_service
                    .fetch(
                        req,
                        self.link
                            .callback(|response: Response<Result<String, Error>>| {
                                if response.status().is_success() {
                                    Msg::Response(Ok("User has been registered".to_string()))
                                } else {
                                    Msg::Response(Err("User could not be registered".to_string()))
                                }
                            }),
                    )
                    .unwrap();
                self.fetch_task = Some(fetch_task);
            }
            Msg::Response(res) => {
                self.response_data = Some(res);
                self.fetch_task = None;
            }
        }
        true
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
        let onsubmit = self.link.callback(|ev: Event| {
            ev.prevent_default();
            Msg::Request
        });
        let oninput_username = self
            .link
            .callback(|ev: InputData| Msg::UpdateUsername(ev.value));
        let oninput_email = self
            .link
            .callback(|ev: InputData| Msg::UpdateEmail(ev.value));
        let oninput_password = self
            .link
            .callback(|ev: InputData| Msg::UpdatePassword(ev.value));
        let oninput_password_repeat = self
            .link
            .callback(|ev: InputData| Msg::UpdatePasswordRepeat(ev.value));
        html! {
            <div class=Classes::from(self.props.class.clone()).extend(self.style.clone())>
                <form id=&self.id class="signup" onsubmit=onsubmit>
                    <label for=format!("{}-email", self.id)>{"Email"}</label>
                    <input
                        id=format!("{}-email", self.id)
                        type="email"
                        value=&self.form_data.email
                        oninput=oninput_email
                        placeholder="Email"
                        spellcheck="false" />
                    <label for=format!("{}-username", self.id)>{"Username"}</label>
                    <input
                        id=format!("{}-username", self.id)
                        type="text"
                        value=&self.form_data.username
                        oninput=oninput_username
                        placeholder="Username"
                        spellcheck="false" />
                    <label for=format!("{}-password", self.id)>{"Password"}</label>
                    <input
                        id=format!("{}-password", self.id)
                        type="password"
                        value=&self.form_data.password
                        oninput=oninput_password
                        placeholder="Password"
                        spellcheck="false" />
                    <label for=format!("{}-password-repeat", self.id)>
                        {"Repeat Password"}
                    </label>
                    <input
                        id=format!("{}-password-repeat", self.id)
                        type="password"
                        value=&self.form_data.password_repeat
                        oninput=oninput_password_repeat
                        placeholder="Repeat Password"
                        spellcheck="false" />
                    <button type="submit" disabled=self.fetch_task.is_some()>
                        { "Sign up" }
                    </button>
                </form>
            </div>
        }
    }
}

impl From<Model> for UserDTO {
    fn from(model: Model) -> Self {
        Self {
            email: model.email,
            username: model.username,
            password: model.password,
            login_session: "".to_string(),
        }
    }
}
