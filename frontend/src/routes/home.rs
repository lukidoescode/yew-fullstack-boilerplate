use css_in_rust::Style;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

/// Home page for logging in or displaying the Dashboard.
pub struct Home {
    style: Style,
}

pub enum Msg {}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            String::from("home-page"),
            String::from(
                r#"
                "#,
            ),
        )
        .expect("An error occured while creating the style.");
        Self { style }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.clone()>
                <h1>{"Home Page"}</h1>
                <p>{"Home Page Content"}</p>
            </div>
        }
    }
}
