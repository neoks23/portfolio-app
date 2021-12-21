use yew::prelude::*;
use stdweb::*;
use std::string;
use crate::app_router::{AppRoute};
use yew_router::prelude::*;

pub enum Msg{
    Click,
    ToggleMusic,
    ToggleSfx,
    ToggleShop,
    ZhongXina,
    EggMan,
    XiJinPing,
    SuperIdol,
    TheWok,
}

pub struct Minigame {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    value: i64,
    social_credit: i64,
    start_music: bool,
    toggle_music: bool,
    toggle_sfx: bool,
    toggle_shop: bool,
    egg_man: i64,
    the_wok: i64,
    super_idol: i64,
    xi_jin_ping: i64,
    log_msg: &'static str,
}

impl Component for Minigame {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            social_credit: 15,
            start_music: true,
            toggle_music: true,
            toggle_sfx: true,
            toggle_shop: true,
            egg_man: 2500,
            the_wok: 10000,
            super_idol: 50000,
            xi_jin_ping: 120000,
            log_msg: "",
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if self.start_music{
            js!{
                var audio = document.getElementById("redsunintheskys");
                audio.loop = true;
                audio.play();
            }
            self.start_music = false;
        }

        match msg{
            Msg::ToggleMusic => {
                self.toggle_music = !self.toggle_music;
                if self.toggle_music{
                    js!{
                        var audio = document.getElementById("redsunintheskys");
                        audio.loop = true;
                        audio.play();
                    }
                        }
                        else{
                            js!{
                        var audio = document.getElementById("redsunintheskys");
                        audio.pause();
                    }
                }
                return true;
            }
            Msg::ToggleSfx => {
                self.toggle_sfx = !self.toggle_sfx;
            },
            Msg::ToggleShop => {
                self.toggle_shop = !self.toggle_shop;
                if self.toggle_shop{
                    js!{
                        document.getElementById("shop-items").style.visibility = "visible";
                    }
                }
                else{
                    js!{
                        document.getElementById("shop-items").style.visibility = "hidden";
                    }
                }
            },
            Msg::ZhongXina =>{
                self.social_credit = 15;
                self.log_msg = "Selected zhong xina (+15 SC)";
                js!{
                    var clicker = document.getElementById("clicker");
                    clicker.src = "../../../assets/images/minigame/johnxina.png";
                }
                return true;
            }
            Msg::EggMan => {

                if self.value > self.egg_man {
                    self.value = self.value - self.egg_man;
                    self.social_credit = 50;
                    js!{
                        var clicker = document.getElementById("clicker");
                        clicker.src = "../../../assets/images/minigame/eggman.png";
                    }
                    if self.egg_man == 0{
                        self.log_msg = "Selected eggman (+50 SC)";
                    }
                    else{
                        self.log_msg = "Purchased eggman (+50 SC)";
                        self.egg_man = 0;
                    }
                }
                else{
                    self.log_msg = "Not enough social credit to purchase egg man";
                }
                return true;
            },
            Msg::TheWok => {
                if self.value > self.the_wok {
                    self.value = self.value - self.the_wok;
                    self.social_credit = 100;
                    js!{
                        var clicker = document.getElementById("clicker");
                        clicker.src = "../../../assets/images/minigame/thewok.png";
                    }
                    if self.the_wok == 0{
                        self.log_msg = "Selected the wok (+100 SC)";
                    }
                    else{
                        self.log_msg = "Purchased the wok (+100 SC)";
                        self.the_wok = 0;
                    }
                }
                else{
                    self.log_msg = "Not enough social credit to purchase the wok";
                }
                return true;
            }
            Msg::SuperIdol => {
                if self.value > self.super_idol {
                    self.value = self.value - self.super_idol;
                    self.social_credit = 200;
                    js!{
                        var clicker = document.getElementById("clicker");
                        clicker.src = "../../../assets/images/minigame/superidol.png";
                    }
                    if self.super_idol == 0{
                        self.log_msg = "Selected super idol (+200 SC)";
                    }
                    else{
                        self.log_msg = "Purchased super idol (+200 SC)";
                        self.super_idol = 0;
                    }
                }
                else{
                    self.log_msg = "Not enough social credit to purchase super idol";
                }
                return true;
            }
            Msg::XiJinPing => {

                if self.value > self.xi_jin_ping {
                    self.value = self.value - self.xi_jin_ping;
                    self.social_credit = 30000000;
                    js!{
                        var clicker = document.getElementById("clicker");
                        clicker.src = "../../../assets/images/minigame/xijinping.png";
                    }
                    if self.xi_jin_ping == 0{
                        self.log_msg = "Selected xi jin ping (+30000000 SC)";
                    }
                    else{
                        self.log_msg = "Purchased xi jin ping (+30000000 SC)";
                        self.xi_jin_ping = 0;
                    }
                }
                else{
                    self.log_msg = "Not enough social credit to purchase xi jin ping";
                }
                return true;
            }
            Msg::Click => {
                self.value = self.value + self.social_credit;

                if self.toggle_sfx{
                    js!{
                        var audio = new Audio("../../../assets/dingsfx.mp3");
                        audio.cloneNode().play();
                    }
                }

                js!{
                    function getRndInteger(min, max) {
                        return Math.floor(Math.random() * (max - min) ) + min;
                    }

                    var socialcredit = document.getElementById("socialcredit");
                    var clicker = document.getElementById("clicker");
                    var element = socialcredit.cloneNode(true);
                    document.getElementById("socialcreditimages").appendChild(element);
                    element.classList.remove("toheaven");
                    element.style.left = getRndInteger(300, 300 + clicker.width) + "px";
                    element.style.top = getRndInteger(300, 100 + clicker.height) + "px";
                    void element.offsetWidth;
                    element.classList.add("toheaven");
                    element.outerHTML;

                };

                return true;
            },
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let history =  ctx.link().history().unwrap();
        let home_history = history.clone();
        let contact_history = history.clone();
        let home = Callback::once(move |_| home_history.push(AppRoute::Index));
        let contact = Callback::once(move |_| contact_history.push(AppRoute::Contact));

        html! {
            <div class="biggerscreen">
                <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                    <Link<AppRoute> classes={"navbar-brand"} to={AppRoute::Index}>
                        <img src="../../../assets/images/bootstrap.svg" width="30" height="30" class="d-inline-block align-top" alt="?" />
                        {" Bootstrap"}
                    </Link<AppRoute>>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {home}>{"Home"}</button>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {contact}>{"Contact"}</button>
                    <div class="custom-control custom-switch nav-switch">
                      <input type="checkbox" checked={self.toggle_sfx} class="custom-control-input" id="customSwitch1" onclick={ctx.link().callback(|_| Msg::ToggleSfx)}/>
                      <label class="custom-control-label selectDisable" for="customSwitch1">{"Sound effects"}</label>
                    </div>
                    <div class="custom-control custom-switch nav-switch selectDisable">
                      <input type="checkbox" checked={self.toggle_music} class="custom-control-input selectDisable" id="customSwitch2" onclick={ctx.link().callback(|_| Msg::ToggleMusic)}/>
                      <label class="custom-control-label selectDisable" for="customSwitch2">{"Music"}</label>
                    </div>
                    <audio id="redsunintheskys" controls=true>
                      <source src="../../../assets/redsunintheskysrock.mp3" type="audio/mpeg" />
                    </audio>
                </nav>
                <div id="totalcredit" class="selectDisable">
                    <h3>{format!("Total social credit: {}", self.value)} </h3>
                </div>
                <div id="shop">
                    <div class="custom-control custom-switch nav-switch selectDisable">
                      <input type="checkbox" checked={self.toggle_shop} class="custom-control-input selectDisable" id="customSwitch3" onclick={ctx.link().callback(|_| Msg::ToggleShop)}/>
                      <label class="custom-control-label selectDisable" for="customSwitch3">{"Shop"}</label>
                    </div>
                    <div id="shop-items">
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::ZhongXina)}>{"Zhong Xina 0 SC"}</button><br/>
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::EggMan)}>{"Eggman 2500 SC"}</button><br/>
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::TheWok)}>{"The wok 10000 SC"}</button><br/>
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::SuperIdol)}>{"Super idol 50000 SC"}</button><br/>
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::XiJinPing)}>{"Xi jin ping 120000 SC"}</button><br/>
                        <h6 class="selectDisable">{self.log_msg}</h6>
                    </div>
                </div>
                <div id="socialcreditimages">
                    <img id="socialcredit" class="toheaven selectDisable" src="../../../assets/images/minigame/socialcredit.jpg" style="position:absolute; top:100px; left:100px; width:200px"/>
                </div>

                <img id="clicker" class="mx-auto d-block fixed-bottom selectDisable" src="../../../assets/images/minigame/johnxina.png" alt="?" height="400px" width="800px" onclick = {ctx.link().callback(|_| Msg::Click)}/>
            </div>
        }
    }
}