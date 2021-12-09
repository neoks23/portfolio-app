use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages;

pub struct AppRouter {}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/testpage"]
    Test,
    #[to = "/"]
    Index,
}

pub type Link = RouterAnchor<AppRoute>;

impl Component for AppRouter {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render_func = Router::render(|route: AppRoute| match route {
            AppRoute::Test => html! { <pages::TestPage/> },
            AppRoute::Index => html! { <pages::Home/> },
        });

        html! {
            <Router<AppRoute, ()> render=render_func/>
        }
    }
}