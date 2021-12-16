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
        let me_history = history.clone();
        let home = Callback::once(move |_| home_history.push(AppRoute::Index));
        let me = Callback::once(move |_| me_history.push(AppRoute::Me));

        html! {
            <>
                <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                    <Link<AppRoute> classes={"navbar-brand"} to={AppRoute::Index}>
                        <img src="../../../assets/images/bootstrap.svg" width="30" height="30" class="d-inline-block align-top" alt="" />
                        {" Bootstrap"}
                    </Link<AppRoute>>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {home}>{"Home"}</button>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {me}>{"Previous slide"}</button>
                </nav>

                <br/><br/><br/>
                <h1 class="text-justify text-xl repo-title">{"Software repository"}</h1>
                <br/>
                <table class="table table-dark repo-table">
                    <thead>
                        <tr>
                            <th scope="col">{"Title"}</th>
                            <th scope="col">{"Description"}</th>
                            <th scope="col">{"Language"}</th>
                            <th scope="col">{"Created at"}</th>
                            <th scope="col">{"Size"}</th>
                            <th scope="col">{"Stars"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <th scope="row" class="repo-btn" onclick={ctx.link().callback(|_| Msg::R1)}>{"Rust invaders"}</th>
                            <td>{"Galaga clone made in the bevy game engine :)"}</td>
                            <td>{"Rust"}</td>
                            <td>{"Sep 9, 2021"}</td>
                            <td>{"43.27 MB"}</td>
                            <td>{"0"}</td>
                        </tr>
                        <tr>
                            <th scope="row" class="repo-btn" onclick={ctx.link().callback(|_| Msg::R2)}>{"Ball physics test"}</th>
                            <td>{"Little 3d ball physics game made in godot"}</td>
                            <td>{"C#"}</td>
                            <td>{"Oct 10, 2021"}</td>
                            <td>{"40.56 MB"}</td>
                            <td>{"0"}</td>
                        </tr>
                        <tr>
                            <th scope="row" class="repo-btn" onclick={ctx.link().callback(|_| Msg::R3)}>{"php-CRUD"}</th>
                            <td>{"php crud database"}</td>
                            <td>{"PHP"}</td>
                            <td>{"Jun 27, 2021"}</td>
                            <td>{"3 KB"}</td>
                            <td>{"0"}</td>
                        </tr>
                        <tr>
                            <th scope="row" class="repo-btn" onclick={ctx.link().callback(|_| Msg::R4)}>{"SuperBuzWorld"}</th>
                            <td>{"2D platformer game made in godot"}</td>
                            <td>{"C#"}</td>
                            <td>{"Apr 04, 2021"}</td>
                            <td>{"22.9 MB"}</td>
                            <td>{"0"}</td>
                        </tr>
                        <tr>
                            <th scope="row" class="repo-btn" onclick={ctx.link().callback(|_| Msg::R5)}>{"Paint app"}</th>
                            <td>{"Paint application made in unity 3D"}</td>
                            <td>{"C#"}</td>
                            <td>{"Jul 19, 2020"}</td>
                            <td>{"17.44 MB"}</td>
                            <td>{"0"}</td>
                        </tr>
                        <tr>
                            <th scope="row" class="repo-btn" onclick={ctx.link().callback(|_| Msg::R6)}>{"3D rainbow cube visualiser"}</th>
                            <td>{"A cube responsive to music made in unity 3D"}</td>
                            <td>{"C#"}</td>
                            <td>{"Jun 28, 2020"}</td>
                            <td>{"21.51 MB"}</td>
                            <td>{"0"}</td>
                        </tr>
                    </tbody>
                </table>
            </>
        }
    }
}