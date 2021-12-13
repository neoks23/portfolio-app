use yew::prelude::*;
use stdweb::*;
use crate::app_router::{AppRoute};
use yew_router::prelude::Link;

pub struct Software {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    value: i64,
}

impl Component for Software {
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
            <>
                <Link<AppRoute> to={AppRoute::Me}>{"Previous slide"}</Link<AppRoute>>
                <Link<AppRoute> to={AppRoute::Index}>{"Back to home panel"}</Link<AppRoute>>
            </>
        }
    }
}