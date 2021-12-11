use yew::prelude::*;
use yew_router::prelude::*;

use stdweb::*;
use crate::app_router::{AppRoute, Link};

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
    link: ComponentLink<Self>,
    value: i64,
    smVisible: bool,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            smVisible: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::P1YT => {
                js! {
                    var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                    audio.cloneNode().play();
                };
                if self.smVisible {
                    js!{
                        window.open("https://www.youtube.com/channel/UCoqfYvqcMCGoYMc8rNAXPBg", "_blank");
                    }
                } else {
                    js!{
                        location.href = "/me";
                    }
                }
                true
            },
            Msg::P2SC => {
                js! {
                    var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                    audio.cloneNode().play();
                };

                if self.smVisible {
                    js!{
                        window.open("https://www.snapchat.com/add/koensa1", "_blank");
                    }
                } else {
                    js!{
                        location.href = "/software";
                    }
                }
                true
            },
            Msg::P3IG => {
                js! {
                    var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                    audio.cloneNode().play();
                };

                if self.smVisible {
                    js!{
                        window.open("https://www.instagram.com/koensa1/", "_blank");
                    }
                } else {

                }
                true
            }
            Msg::SMToggle => {
                self.smVisible = !self.smVisible;
                if self.smVisible{
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
                } else {
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
                };

                if self.smVisible {
                    js!{
                        window.open("https://www.linkedin.com/in/koen-sampers-8345741a5/", "_blank");
                    }
                } else {

                }
                true
            }
            Msg::P5DISC => {
                js! {
                    var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                    audio.cloneNode().play();
                };

                if self.smVisible {
                    js!{
                        window.open("https://discord.gg/nxvjnHrr", "_blank");
                    }
                } else {

                }
                true
            }
            Msg::P6GIT => {
                js! {
                    var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                    audio.cloneNode().play();
                };

                if self.smVisible {
                    js!{
                        window.open("https://github.com/neoks23", "_blank");
                    }
                } else {

                }
                true
            }
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }
    fn view(&self) -> Html {
        html! {
            <>
                <div class="centerdiv">
                    <div class="menubox">
                        <div class="hexagon normalHexSize hexpos1" onclick=self.link.callback(|_| Msg::P1YT) >
                            <p id="hextext1" class="noselect">{ "1"  }</p>
                            <img id="heximg1" src="../../../assets/logos2/social-media-222/png-64/youtube-64x64-3649993.png" alt="globe" width="32px" class="heximg"/>
                        </div>
                        <div class="hexagon normalHexSize hexpos2" onclick=self.link.callback(|_| Msg::P2SC)>
                            <p id="hextext2" class="noselect">{ "2"  }</p>
                            <img id="heximg2" src="../../../assets/logos2/social-media-222/png-64/snapchat-64x64-3649983.png" alt="globe" width="32px" class="heximg"/>
                        </div>
                        <div class="hexagon normalHexSize hexpos3" onclick=self.link.callback(|_| Msg::P3IG)>
                            <p id="hextext3" class="noselect">{ "3" }</p>
                            <img id="heximg3" src="../../../assets/logos2/social-media-222/png-64/instagram-64x64-3649976.png" alt="globe" width="32px" class="heximg"/>
                        </div>
                        <div class="hexagon normalHexSize hexpos4"  onclick=self.link.callback(|_| Msg::SMToggle)>
                            <img src="../../../assets/logos2/social-media-222/png-64/dribbble-64x64-3649973.png" alt="globe" width="32px" class="image"/>
                        </div>

                        <div class="hexagon normalHexSize hexpos5" onclick=self.link.callback(|_| Msg::P4LI)>
                            <p id="hextext5" class="noselect">{ "5"  }</p>
                            <img id="heximg5" src="../../../assets/logos2/social-media-222/png-64/linkedin-64x64-3649977.png" alt="globe" width="32px" class="heximg"/>
                        </div>
                        <div class="hexagon normalHexSize hexpos6" onclick=self.link.callback(|_| Msg::P5DISC)>
                            <p id="hextext6" class="noselect">{ "6"  }</p>
                            <img id="heximg6" src="../../../assets/logos2/social-media-222/png-64/discord-64x64-3649972.png" alt="globe" width="32px" class="heximg"/>
                        </div>
                        <div class="hexagon normalHexSize hexpos7" onclick=self.link.callback(|_| Msg::P6GIT)>
                            <p id="hextext7" class="noselect">{ "7" }</p>
                            <img id="heximg7" src="../../../assets/images/github.png" alt="globe" width="32px" class="heximg"/>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}