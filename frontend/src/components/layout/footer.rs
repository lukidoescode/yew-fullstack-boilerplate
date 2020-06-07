use css_in_rust::Style;
use yew::{html, Classes, Component, ComponentLink, Html, Properties, ShouldRender};

/// Footer e.g. for displaying copyright notice and version info.
pub struct Footer {
    props: Props,
    style: Style,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: String,
}

pub enum Msg {}

impl Component for Footer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            String::from("footer"),
            String::from(
                r#"
                background-color: rgb(var(--color-fg));
                color: rgb(var(--color-bg));
                padding: 7px 5px 2px 5px;
                "#,
            ),
        )
        .expect("An error occured while creating the style.");
        Self { props, style }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
                <p>{"Yew Fullstack Boilerplate"}</p>
            </div>
        }
    }
}
