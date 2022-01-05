use yew::prelude::*;
use stdweb::*;
use crate::app_router::{AppRoute};
use yew_router::prelude::*;

pub struct Credits {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    value: i64,
}

impl Component for Credits {
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
            <div class="biggerscreen">
                <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                    <Link<AppRoute> classes={"navbar-brand"} to={AppRoute::Index}>
                        <img src="../../../assets/images/bootstrap.svg" width="30" height="30" class="d-inline-block align-top" alt="?" />
                        {" Bootstrap"}
                    </Link<AppRoute>>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {home}>{"Home"}</button>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {me}>{"About me"}</button>
                </nav>

                <div class="credits">
                    <h1 class="text-xl">{"Toolchains"}</h1><br/>
                    <h3 class="text-xl">{"This website was made possible with:"}</h3>
                    <div>
                        <a href="https://getbootstrap.com/">
                            <img src="../../../assets/images/bootstrap.svg" width="100" height="100" class="credits-v"/>
                        </a>
                        <h3 class="text-xl">{"Bootstrap"}</h3>
                    </div>
                    <div>
                        <a href="https://www.rust-lang.org/" >
                            <img src="../../../assets/images/credits/rustlogo.png" width="100" height="100" class="credits-v"/>
                        </a>
                        <h3 class="text-xl">{"Rust"}</h3>
                    </div>
                    <div>
                        <a href="https://www.javascript.com/">
                            <img src="../../../assets/images/credits/javascript-logo.png" width="100" height="100" class="credits-v"/>
                        </a>
                        <h3 class="text-xl">{"Javascript"}</h3>
                    </div>

                    <div>
                        <a href="https://yew.rs/">
                            <img src="../../../assets/images/credits/yew-logo.png" width="100" height="100" class="credits-v"/>
                        </a>
                        <h3 class="text-xl">{"Yew"}</h3>
                    </div>
                    <div>
                        <a href="https://webassembly.org/">
                            <img src="../../../assets/images/credits/webassembly-logo.png" width="100" height="100" class="credits-v" />
                        </a>
                        <h3 class="text-xl">{"Webassembly"}</h3>
                    </div>
                    <br/><br/>
                    <div>
                        <h3 class="text-xl credits-v">{"Website is deployed and hosted on:"}</h3>
                        <a href="https://www.heroku.com/">
                            <img src="../../../assets/images/credits/heroku-vector-logo.svg" width="200" height="200" />
                        </a>
                    </div>
                </div>
            </div>
        }
    }
}