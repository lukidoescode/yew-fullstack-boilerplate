use crate::routes::{AppRoutes, Home, Profile};
// use std::marker::PhantomData;
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::switch::Permissive;
use yew_router::{route::Route as YewRoute, router::Router as YewRouter};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct Router<NotFound: Component + 'static> {
    props: Props,
    link: ComponentLink<Self>,
}

impl<NotFound: Component + 'static> Component for Router<NotFound> {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
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
                        AppRoutes::Home => html!{ <Home /> },
                        AppRoutes::Profile => html!{ <Profile /> },
                        AppRoutes::NotFound(Permissive(None)) => html!{ <NotFound /> },
                        AppRoutes::NotFound(Permissive(Some(_missed_route))) => html!{ <NotFound /> },
                    }
                })
                redirect = YewRouter::redirect(|route: YewRoute| {
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
