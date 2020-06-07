use crate::{
    agents::auth::AuthEventBus,
    components::auth::{AuthControls, Login, Signup},
    model::Auth as AuthModel,
    routes::AppRoutes,
};
use css_in_rust::Style;
use yew::agent::{Bridge, Bridged};
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::prelude::*;

/// Header with menu and user controls.
pub struct Header {
    props: Props,
    style: Style,
    auth: Option<AuthModel>,
    _auth_event_bus_producer: Box<dyn Bridge<AuthEventBus>>,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
}

#[derive(PartialEq, Debug)]
pub enum Msg {
    AuthEvent(Option<AuthModel>),
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create("header", include_str!("header.scss"))
            .expect("An error occured while creating the style.");
        let auth_cb = link.callback(Msg::AuthEvent);
        let auth_producer = AuthEventBus::bridge(auth_cb);
        Self {
            props,
            style,
            auth: None,
            _auth_event_bus_producer: auth_producer,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AuthEvent(auth) => {
                self.auth = auth;
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
        html! {
            <div class=Classes::from(self.props.class.clone()).extend(self.style.clone())>
                <div class="menu">
                    <RouterAnchor<AppRoutes> route=AppRoutes::Home>
                        { "Home" }
                    </RouterAnchor<AppRoutes>>
                    <RouterAnchor<AppRoutes> route=AppRoutes::Profile>
                        { "Profile" }
                    </RouterAnchor<AppRoutes>>
                </div>
                <div class="divider"/>
                <div class="auth-info">
                    { self.render_auth() }
                </div>
            </div>
        }
    }
}

impl Header {
    fn render_auth(&self) -> Html {
        match &self.auth {
            // TODO add logout option
            Some(auth) => html! {
                <AuthControls auth=auth />
            },
            None => html! {
                <>
                    <Login class="login" />
                    <Signup class="signup" />
                </>
            },
        }
    }
}
