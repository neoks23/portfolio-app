use yew::prelude::*;
use stdweb::*;
use crate::app_router::{AppRoute};
use yew_router::prelude::*;

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
        let history =  ctx.link().history().unwrap();
        let home_history = history.clone();
        let me_history = history.clone();
        let home = Callback::once(move |_| home_history.push(AppRoute::Index));
        let me = Callback::once(move |_| me_history.push(AppRoute::Me));

        html! {
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <Link<AppRoute> classes={"navbar-brand"} to={AppRoute::Index}>
                    <img src="../../../assets/images/bootstrap.svg" width="30" height="30" class="d-inline-block align-top" alt="" />
                    {" Bootstrap"}
                </Link<AppRoute>>
                <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {home}>{"Go Home"}</button>
                <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {me}>{"Go to previous slide"}</button>
            </nav>
        }
    }
}