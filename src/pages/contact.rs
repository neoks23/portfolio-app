use yew::prelude::*;
use yew::events::{InputEvent};
use stdweb::*;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef};
use crate::app_router::{AppRoute};
use yew_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;

use reqwasm::http::*;
use reqwasm::http::{Method};


pub enum Msg{
    EmailChanged,
    MessageChanged,
    SubmitForm,
}

pub struct Contact {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    email_input: NodeRef,
    message_input: NodeRef,
    email: String,
    message: String,
    out: String,
}

impl Component for Contact {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            email_input:  NodeRef::default(),
            message_input:  NodeRef::default(),
            email: "".to_string(),
            message: "".to_string(),
            out: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            Msg::EmailChanged => {
                if let Some(input) = self.email_input.cast::<HtmlInputElement>() {
                    let value = input.value();

                    self.email = value.to_string();

                    return true;
                } else {
                    return false;
                }
            },
            Msg::MessageChanged => {
                if let Some(input) = self.message_input.cast::<HtmlInputElement>() {
                    let value = input.value();

                    self.message = value.to_string();

                    return true;
                } else {
                    return false;
                }
            },
            Msg::SubmitForm =>{
                self.out = "houbabla".to_string();

                spawn_local(async move {
                    let resp = Request::get(&format!("https://formspree.io/neoks23@gmail.com"))
                        .method(Method::POST)
                        .header("Access-Control-Allow-Origin", "null")
                        .header("Accept", "application/json")
                        .body(
                            serde_json::to_string("email: \"a.visitor@email.com\", message: \"Hello\" ")
                            .unwrap(),
                        )
                        .send()
                        .await
                        .unwrap();
                });
                return true;
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let history =  ctx.link().history().unwrap();
        let home_history = history.clone();
        let credits_history = history.clone();
        let home = Callback::once(move |_| home_history.push(AppRoute::Index));
        let credits = Callback::once(move |_| credits_history.push(AppRoute::Credits));

        let email_onchange = ctx.link().callback(|_| Msg::EmailChanged);
        let message_onchange = ctx.link().callback(|_| Msg::MessageChanged);

        html! {
            <div class="biggerscreen">
                <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                    <Link<AppRoute> classes={"navbar-brand"} to={AppRoute::Index}>
                        <img src="../../../assets/images/bootstrap.svg" width="30" height="30" class="d-inline-block align-top" alt="?" />
                        {" Bootstrap"}
                    </Link<AppRoute>>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {home}>{"Home"}</button>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button" onclick = {credits}>{"Credits"}</button>
                </nav>

                <form class="contact-form" method="POST" action="http://formspree.io/f/mgedvrew">
                    <label for="email">{"Email address"}</label>
                    <input ref={self.email_input.clone()} onchange={email_onchange} type="email" name="email" class="form-control" id="email" aria-describedby="emailHelp" placeholder="Enter email" />
                    <label for="message">{"Message"}</label>
                    <textarea ref={self.message_input.clone()} onchange={message_onchange} class="form-control" name="message" placeholder="Subject:

Message:" style="height:200px"></textarea>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" type="submit">{"Submit"}</button>
                </form>
            </div>
        }
    }
}