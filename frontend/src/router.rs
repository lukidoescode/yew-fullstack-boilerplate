use crate::routes::{AppRoutes, Home, Profile};
// use std::marker::PhantomData;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::switch::Permissive;
use yew_router::{route::Route, router::Router as YewRouter};

pub struct Router {}

impl Component for Router {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <YewRouter<AppRoutes>
                render=YewRouter::render(|switch: AppRoutes| {
                    match switch {
                        AppRoutes::Home => html!{<Home />},
                        AppRoutes::Profile => html!{<Profile />},
                        AppRoutes::NotFound(Permissive(None)) => html!{"Page not found"},
                        AppRoutes::NotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                    }
                })
                redirect = YewRouter::redirect(|route: Route| {
                    AppRoutes::NotFound(Permissive(Some(route.route)))
                })
            />
        }
    }
}

// pub struct Protected<T: Component + 'static> {
//     props: T::Properties,
//     phantom: PhantomData<&'static T>,
// }

// impl<T> Component for Protected<T>
// where
//     T: Component + 'static,
// {
//     type Message = T::Message;
//     type Properties = T::Properties;

//     fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
//         Self {
//             props: props,
//             phantom: PhantomData,
//         }
//     }

//     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
//         true
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         true
//     }

//     fn view(&self) -> Html {
//         html! {<T />}
//     }
// }
