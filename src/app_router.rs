use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages;
use yew_router::__macro::Router;

pub struct AppRouter {}

#[derive(Routable, PartialEq, Debug, Clone)]
pub enum AppRoute {
    #[at("/me")]
    Me,
    #[at("/software")]
    Software,
    #[at("/")]
    Index,
}

pub fn switch_main(route: &AppRoute) -> Html{
    match route.clone() {
        AppRoute::Me => html!{ <pages::Me/> },
        AppRoute::Software => html! { <pages::Software/> },
        AppRoute::Index => html! { <pages::Home/> },
    }
}
//pub type Link = RouterAnchor<AppRoute>;

impl Component for AppRouter {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<AppRoute> render={Switch::render(switch_main)} />
            </BrowserRouter>
        }
    }
}
