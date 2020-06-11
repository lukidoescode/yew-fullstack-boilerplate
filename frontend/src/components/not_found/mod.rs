use css_in_rust::Style;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

/// Not found page content.
pub struct NotFound {
    style: Style,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub enum Msg {}

impl Component for NotFound {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create("not-found", include_str!("style.scss"))
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
                <h1>{ "Not Found" }</h1>
            </div>
        }
    }
}
