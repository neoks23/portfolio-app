use yew::prelude::*;
use yew_router::prelude::*;

use stdweb::*;
use crate::app_router::{AppRoute};

pub enum Msg {
    P1YT,
    P2SC,
    P3IG,
    SMToggle,
    P4LI,
    P5DISC,
    P6GIT,
}

pub struct Home {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    value: i64,
    sm_visible: bool,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {

            value: 0,
            sm_visible: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::P1YT => {
                js! {
                    var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                    audio.cloneNode().play();
                    window.open("https://www.youtube.com/channel/UCoqfYvqcMCGoYMc8rNAXPBg", "_blank");
                };
                true
            },
            Msg::P2SC => {
                js! {
                    var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                    audio.cloneNode().play();
                    window.open("https://www.snapchat.com/add/koensa1", "_blank");
                };
                true
            },
            Msg::P3IG => {
                js! {
                    var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                    audio.cloneNode().play();
                    window.open("https://www.instagram.com/koensa1/", "_blank");
                };
                true
            },
            Msg::SMToggle => {
                self.sm_visible = !self.sm_visible;
                if self.sm_visible {
                    js!{
                        var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                        audio.cloneNode().play();
                        document.getElementById("hextext1").style.visibility = "hidden";
                        document.getElementById("heximg1").style.visibility = "visible";
                        document.getElementById("hextext2").style.visibility = "hidden";
                        document.getElementById("heximg2").style.visibility = "visible";
                        document.getElementById("hextext3").style.visibility = "hidden";
                        document.getElementById("heximg3").style.visibility = "visible";
                        document.getElementById("hextext5").style.visibility = "hidden";
                        document.getElementById("heximg5").style.visibility = "visible";
                        document.getElementById("hextext6").style.visibility = "hidden";
                        document.getElementById("heximg6").style.visibility = "visible";
                        document.getElementById("hextext7").style.visibility = "hidden";
                        document.getElementById("heximg7").style.visibility = "visible";
                    };
                }

                if !self.sm_visible {
                    js!{
                        var audio = new Audio("../../../assets/UI_Quirky8.mp3");
                        audio.cloneNode().play();
                        document.getElementById("hextext1").style.visibility = "visible";
                        document.getElementById("heximg1").style.visibility = "hidden";
                        document.getElementById("hextext2").style.visibility = "visible";
                        document.getElementById("heximg2").style.visibility = "hidden";
                        document.getElementById("hextext3").style.visibility = "visible";
                        document.getElementById("heximg3").style.visibility = "hidden";
                        document.getElementById("hextext5").style.visibility = "visible";
                        document.getElementById("heximg5").style.visibility = "hidden";
                        document.getElementById("hextext6").style.visibility = "visible";
                        document.getElementById("heximg6").style.visibility = "hidden";
                        document.getElementById("hextext7").style.visibility = "visible";
                        document.getElementById("heximg7").style.visibility = "hidden";
                    };
                }

                true
            },
            Msg::P4LI => {
                js! {
                    var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                    audio.cloneNode().play();
                    window.open("https://www.linkedin.com/in/koen-sampers-8345741a5/", "_blank");
                };
                true
            }
            Msg::P5DISC => {
                js! {
                    var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                    audio.cloneNode().play();
                    window.open("https://discord.gg/nxvjnHrr", "_blank");
                };
                true
            }
            Msg::P6GIT => {
                js! {
                    var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                    audio.cloneNode().play();
                    window.open("https://github.com/neoks23", "_blank");
                };
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {

        let history =  ctx.link().history().unwrap();
        let me_history = history.clone();
        let software_history = history.clone();
        let me = Callback::once(move |_| me_history.push(AppRoute::Me));
        let software = Callback::once(move |_| software_history.push(AppRoute::Software));


        html! {
            <>
                <div class="centerdiv">
                    <div class="menubox">
                        <div class="hexagon normalHexSize hexpos1" onclick={if self.sm_visible { ctx.link().callback(|_| Msg::P1YT)} else {me}} >
                            <p id="hextext1" class="noselect">{ "1"  }</p>
                            <img id="heximg1" src="../../../assets/images/youtube-64x64-3649993.png" alt="globe" width="32px" class="heximg" />
                        </div>
                        <div class="hexagon normalHexSize hexpos2" onclick={if self.sm_visible {ctx.link().callback(|_| Msg::P2SC)} else {software}}>
                            <p id="hextext2" class="noselect">{ "2"  }</p>
                            <img id="heximg2" src="../../../assets/images/snapchat-64x64-3649983.png" alt="globe" width="32px" class="heximg" />
                        </div>
                        <div class="hexagon normalHexSize hexpos3" onclick={ctx.link().callback(|_| Msg::P3IG)}>
                            <p id="hextext3" class="noselect">{ "3" }</p>
                            <img id="heximg3" src="../../../assets/images/instagram-64x64-3649976.png" alt="globe" width="32px" class="heximg" />
                        </div>
                        <div class="hexagon normalHexSize hexpos4"  onclick={ctx.link().callback(|_| Msg::SMToggle)}>
                            <img src="../../../assets/images/dribbble-64x64-3649973.png" alt="globe" width="32px" class="image" />
                        </div>

                        <div class="hexagon normalHexSize hexpos5" onclick={ctx.link().callback(|_| Msg::P4LI)}>
                            <p id="hextext5" class="noselect">{ "5"  }</p>
                            <img id="heximg5" src="../../../assets/images/linkedin-64x64-3649977.png" alt="globe" width="32px" class="heximg" />
                        </div>
                        <div class="hexagon normalHexSize hexpos6" onclick={ctx.link().callback(|_| Msg::P5DISC)}>
                            <p id="hextext6" class="noselect">{ "6"  }</p>
                            <img id="heximg6" src="../../../assets/images/discord-64x64-3649972.png" alt="globe" width="32px" class="heximg" />
                        </div>
                        <div class="hexagon normalHexSize hexpos7" onclick={ctx.link().callback(|_| Msg::P6GIT)}>
                            <p id="hextext7" class="noselect">{ "7" }</p>
                            <img id="heximg7" src="../../../assets/images/github.png" alt="globe" width="32px" class="heximg" />
                        </div>
                    </div>
                </div>
            </>
        }
    }
}