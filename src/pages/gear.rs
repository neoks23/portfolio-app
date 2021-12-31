use yew::prelude::*;
use stdweb::*;
use crate::app_router::{AppRoute};
use yew_router::prelude::*;

pub struct Gear {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    value: i64,
}

impl Component for Gear {
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
        let minigame_history = history.clone();
        let home = Callback::once(move |_| home_history.push(AppRoute::Index));
        let minigame = Callback::once(move |_| minigame_history.push(AppRoute::Minigame));

        html! {
            <div class="biggerscreen">
                <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                    <Link<AppRoute> classes={"navbar-brand"} to={AppRoute::Index}>
                        <img src="../../../assets/images/bootstrap.svg" width="30" height="30" class="d-inline-block align-top" alt="?" />
                        {" Bootstrap"}
                    </Link<AppRoute>>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {home}>{"Home"}</button>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {minigame}>{"Minigame"}</button>
                </nav>
                <div class="gear">
                    <ul>
                        <h3>{"Software"}</h3><br/>
                        <li><h4>{"Photoshop CS6"}</h4></li>
                        <li><h4>{"Jetbrains Student license"}</h4></li><br/><br/>
                        <h3>{"Hardware"}</h3><br/>
                        <li><h4>{"Laptop(Lenovo Ideapad 330)"}</h4></li>
                        <li><h4>{"Keyboard(Dierya DK61E)"}</h4></li>
                        <li><h4>{"Monitor(iiyama Black Hawk G2740HSU)"}</h4></li>
                        <li><h4>{"GPU(MSI Radeon RX 580 ARMOR 8G OC)"}</h4></li>
                        <li><h4>{"CPU(AMD Ryzen 5 2400G Boxed)"}</h4></li>
                    </ul>
                </div>
            </div>
        }
    }
}