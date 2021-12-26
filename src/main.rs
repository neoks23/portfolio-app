#![recursion_limit = "1024"]

use yew::prelude::*;
use stdweb::*;
mod pages;
mod app_router;

use app_router::AppRouter;

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    value: i64,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <AppRouter/>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}