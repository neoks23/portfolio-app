use yew::prelude::*;
use stdweb::*;
use crate::app_router::{AppRoute};
use yew_router::prelude::*;

pub enum Msg{
    Click,
    ToggleSfx,
    ToggleShop,
    ZhongXina,
    EggMan,
    XiJinPing,
}

pub struct Minigame {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    value: i64,
    social_credit: i64,
    toggle_sfx: bool,
    toggle_shop: bool,
    egg_man: i64,
    xi_jin_ping: i64,
}

impl Component for Minigame {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            social_credit: 15,
            toggle_sfx: true,
            toggle_shop: true,
            egg_man: 2500,
            xi_jin_ping: 10000,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        js!{
            var audio = document.getElementById("redsunintheskys");
            audio.loop = true;
            audio.play();
        };
        match msg{
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
                }
                return true;
            },
            Msg::XiJinPing => {

                if self.value > self.xi_jin_ping {
                    self.value = self.value - self.xi_jin_ping;
                    self.social_credit = 100;
                    js!{
                        var clicker = document.getElementById("clicker");
                        clicker.src = "../../../assets/images/minigame/xijinping.png";
                    }
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

                    var element = document.getElementById("socialcredit");
                    var clicker = document.getElementById("clicker");
                    element.classList.remove("toheaven");
                    element.style.left = getRndInteger(300, 300 + clicker.width) + "px";
                    element.style.top = getRndInteger(300, 100 + clicker.height) + "px";
                    void element.offsetWidth;
                    element.classList.add("toheaven");

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
                    <audio id="redsunintheskys" controls=true>
                      <source src="../../../assets/redsunintheskysrock.mp3" type="audio/mpeg" />
                    </audio>
                </nav>
                <div id="totalcredit" class="selectDisable">
                    <h3>{format!("Total social credit: {}", self.value)} </h3>
                </div>
                <div id="shop">
                    <div class="custom-control custom-switch nav-switch selectDisable">
                      <input type="checkbox" checked={self.toggle_shop} class="custom-control-input selectDisable" id="customSwitch2" onclick={ctx.link().callback(|_| Msg::ToggleShop)}/>
                      <label class="custom-control-label selectDisable" for="customSwitch2">{"Shop"}</label>
                    </div>
                    <div id="shop-items">
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::ZhongXina)}>{"Zhong Xina 0 SC"}</button><br/>
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::EggMan)}>{"Eggman 2500 SC"}</button><br/>
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::XiJinPing)}>{"Xi jin ping 10000 SC"}</button><br/>
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable">{"Contact"}</button><br/>
                        <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable">{"Contact"}</button><br/>
                    </div>
                </div>
                <img id="socialcredit" class="toheaven selectDisable" src="../../../assets/images/minigame/socialcredit.jpg" style="position:absolute; top:100px; left:100px; width:200px"/>
                <img id="clicker" class="mx-auto d-block fixed-bottom selectDisable" src="../../../assets/images/minigame/johnxina.png" alt="?" height="600px" width="1000px" onclick = {ctx.link().callback(|_| Msg::Click)}/>
            </div>
        }
    }
}