use yew::prelude::*;
use stdweb::*;
use crate::app_router::{AppRoute};
use yew_router::prelude::*;

pub struct Software {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    value: i64,
}

pub enum Msg {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
}

impl Component for Software {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            Msg::R1 => {
                js!{
                    window.open("https://github.com/neoks23/rustinvaders", "_blank");
                }
            },
            Msg::R2 => {
                js!{
                    window.open("https://github.com/neoks23/BallPhysicsTest", "_blank");
                }
            },
            Msg::R3 => {
                js!{
                    window.open("https://github.com/neoks23/php-CRUD", "_blank");
                }
            },
            Msg::R4 => {
                js!{
                    window.open("https://github.com/neoks23/SuperBuzWorld", "_blank");
                }
            },
            Msg::R5 => {
                js!{
                    window.open("https://github.com/neoks23/PaintApp", "_blank");
                }
            },
            Msg::R6 => {
                js!{
                    window.open("https://github.com/neoks23/3DRainbowCubeVisualizer", "_blank");
                }
            },
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let history =  ctx.link().history().unwrap();
        let home_history = history.clone();
        let gear_history = history.clone();
        let home = Callback::once(move |_| home_history.push(AppRoute::Index));
        let gear = Callback::once(move |_| gear_history.push(AppRoute::Gear));

        html! {
            <>
                <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                    <Link<AppRoute> classes={"navbar-brand"} to={AppRoute::Index}>
                        <img src="../../../assets/images/bootstrap.svg" width="30" height="30" class="d-inline-block align-top" alt="" />
                        {" Bootstrap"}
                    </Link<AppRoute>>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {home}>{"Home"}</button>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {gear}>{"Gear"}</button>
                </nav>

                <div class="software">
                    <br/><br/><br/>
                        <h1 class="text-xl">{"Software repository"}</h1>
                    <br/>
                    <a href="https://github.com/anuraghazra/github-readme-stats">
                      <img align="center" class="responsive" src="https://github-readme-stats.vercel.app/api/top-langs/?username=neoks23&layout=compact&theme=radical" />
                    </a><br/><br/>
                    <a href="https://github.com/anuraghazra/github-readme-stats">
                      <img align="center" class="responsive"  src="https://github-readme-stats.vercel.app/api?username=neoks23&show_icons=true&theme=radical" />
                    </a>
                </div>
            </>
        }
    }
}