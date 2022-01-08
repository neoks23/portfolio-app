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
    PlainCookies,
    SugarCookies,
    PeanutButterCookies,
    Oreos,
    ReesesPuffs,
}

pub struct Minigame {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    value: i64,
    cookies: i64,
    start_music: bool,
    toggle_music: bool,
    toggle_sfx: bool,
    toggle_shop: bool,
    sugar_cookies: i64,
    peanut_butter_cookies: i64,
    oreos: i64,
    reeses_puffs: i64,
    log_msg: &'static str,
}

impl Component for Minigame {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            cookies: 15,
            start_music: true,
            toggle_music: true,
            toggle_sfx: true,
            toggle_shop: true,
            sugar_cookies: 2500,
            peanut_butter_cookies: 10000,
            oreos: 50000,
            reeses_puffs: 120000,
            log_msg: "",
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if self.start_music{
            js!{
                var audio = document.getElementById("monke");
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
                        var audio = document.getElementById("monke");
                        audio.loop = true;
                        audio.play();
                    }
                } else {
                    js!{
                        var audio = document.getElementById("monke");
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
            Msg::PlainCookies =>{
                self.cookies = 15;
                self.log_msg = "Selected plain cookies +15";
                js!{
                    var clicker = document.getElementById("clicker");
                    clicker.src = "../../../assets/images/cookieclicker/cookie.png";
                }
                return true;
            }
            Msg::SugarCookies => {

                if self.value > self.sugar_cookies {
                    self.value = self.value - self.sugar_cookies;
                    self.cookies = 50;
                    js!{
                        var clicker = document.getElementById("clicker");
                        clicker.src = "../../../assets/images/cookieclicker/sugar-cookie.png";
                    }
                    if self.sugar_cookies == 0{
                        self.log_msg = "Selected sugar cookies +50";
                    }
                    else{
                        self.log_msg = "Purchased sugar cookies +50";
                        self.sugar_cookies = 0;
                    }
                }
                else{
                    self.log_msg = "Not enough cookies to purchase sugar cookies";
                }
                return true;
            },
            Msg::PeanutButterCookies => {
                if self.value > self.peanut_butter_cookies {
                    self.value = self.value - self.peanut_butter_cookies;
                    self.cookies = 100;
                    js!{
                        var clicker = document.getElementById("clicker");
                        clicker.src = "../../../assets/images/cookieclicker/peanutbuttercookie'.png";
                    }
                    if self.peanut_butter_cookies == 0{
                        self.log_msg = "Selected peanut butter cookies +100";
                    }
                    else{
                        self.log_msg = "Purchased peanut butter cookies +100";
                        self.peanut_butter_cookies = 0;
                    }
                }
                else{
                    self.log_msg = "Not enough cookies to purchase peanut butter cookies";
                }
                return true;
            }
            Msg::Oreos => {
                if self.value > self.oreos {
                    self.value = self.value - self.oreos;
                    self.cookies = 200;
                    js!{
                        var clicker = document.getElementById("clicker");
                        clicker.src = "../../../assets/images/cookieclicker/oreo.png";
                    }
                    if self.oreos == 0{
                        self.log_msg = "Selected oreos +200";
                    }
                    else{
                        self.log_msg = "Purchased oreos +200";
                        self.oreos = 0;
                    }
                }
                else{
                    self.log_msg = "Not enough cookies to purchase oreos";
                }
                return true;
            }
            Msg::ReesesPuffs => {

                if self.value > self.reeses_puffs {
                    self.value = self.value - self.reeses_puffs;
                    self.cookies = 500;
                    js!{
                        var clicker = document.getElementById("clicker");
                        clicker.src = "../../../assets/images/cookieclicker/reesespuffs.png";
                    }
                    if self.reeses_puffs == 0{
                        self.log_msg = "Selected reeses puffs +500";
                    }
                    else{
                        self.log_msg = "Purchased reeses puffs +500";
                        self.reeses_puffs = 0;
                    }
                }
                else{
                    self.log_msg = "Not enough cookies to purchase reeses puffs";
                }
                return true;
            }
            Msg::Click => {
                self.value = self.value + self.cookies;

                if self.toggle_sfx{
                    js!{
                        var audio = new Audio("../../../assets/pop.mp3");
                        audio.cloneNode().play();
                    }
                }

                js!{
                    function getRndInteger(min, max) {
                        return Math.floor(Math.random() * (max - min) ) + min;
                    }

                    var socialcredit = document.getElementById("cookie");
                    var clicker = document.getElementById("clicker");
                    var element = socialcredit.cloneNode(true);
                    document.getElementById("cookies").appendChild(element);
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
                    <audio id="monke" controls=true>
                      <source src="../../../assets/monke.mp3" type="audio/mpeg" />
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
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::PlainCookies)}>{"Plain Cookies 0 Cookies"}</button><br/>
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::SugarCookies)}>{"Sugar Cookies 2500 Cookies"}</button><br/>
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::PeanutButterCookies)}>{"Peanut Butter Cookies 10000 Cookies"}</button><br/>
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::Oreos)}>{"Oreos 50000 Cookies"}</button><br/>
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::ReesesPuffs)}>{"Reeses Puffs 120000 Cookies"}</button><br/>
                        <h6 class="selectDisable">{self.log_msg}</h6>
                    </div>
                </div>
                <div id="cookies">
                    <img id="cookie" class="toheaven selectDisable" src="../../../assets/images/cookieclicker/cookie.png" style="position:absolute; top:100px; left:100px; width:200px"/>
                </div>

                <img id="clicker" class="mx-auto d-block fixed-bottom selectDisable" src="../../../assets/images/cookieclicker/cookie.png" alt="?" height="400px" width="400px" onclick = {ctx.link().callback(|_| Msg::Click)}/>
            </div>
        }
    }
}