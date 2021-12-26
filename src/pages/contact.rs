use yew::prelude::*;
use yew::events::{InputEvent};
use stdweb::*;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, NodeRef};
use crate::app_router::{AppRoute};
use yew_router::prelude::*;
use wasm_bindgen::JsCast;

//SENDGRID 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

pub enum Msg{
    NameChanged,
    EmailChanged,
    SubjectChanged,
    MessageChanged,
    SubmitForm,
}

pub struct Contact {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    name_input: NodeRef,
    email_input: NodeRef,
    subject_input: NodeRef,
    message_input: NodeRef,
    name: String,
    email: String,
    subject: String,
    message: String,
    out: String,
}

impl Component for Contact {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name_input:  NodeRef::default(),
            email_input:  NodeRef::default(),
            subject_input:  NodeRef::default(),
            message_input:  NodeRef::default(),
            name: "".to_string(),
            email: "".to_string(),
            subject: "".to_string(),
            message: "".to_string(),
            out: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            Msg::NameChanged => {
                if let Some(input) = self.name_input.cast::<HtmlInputElement>() {
                    let value = input.value();

                    self.name = value.to_string();

                    return true;
                } else {
                    return false;
                }
            },
            Msg::EmailChanged => {
                if let Some(input) = self.email_input.cast::<HtmlInputElement>() {
                    let value = input.value();

                    self.email = value.to_string();

                    return true;
                } else {
                    return false;
                }
            },
            Msg::SubjectChanged => {
                if let Some(input) = self.subject_input.cast::<HtmlInputElement>() {
                    let value = input.value();

                    self.subject = value.to_string();

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

                /*
                let email = Message::builder()
                    .from(format!("{} <{}>",self.name, self.email).parse().unwrap())
                    .to("Koen Sampers <neoks23@gmail.com>".parse().unwrap())
                    .subject(self.subject)
                    .body(self.message)
                    .unwrap();

                let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

                // Open a remote connection to gmail
                let mailer = SmtpTransport::relay("smtp.gmail.com")
                    .unwrap()
                    .credentials(creds)
                    .build();

                // Send the email
                match mailer.send(&email) {
                    Ok(_) => self.out = "Email sent successfully!".to_string(),
                    Err(e) => self.out = format!("Could not send email: {:?}", e),
                }*/
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

        let name_onchange = ctx.link().callback(|_| Msg::NameChanged);
        let email_onchange = ctx.link().callback(|_| Msg::EmailChanged);
        let subject_onchange = ctx.link().callback(|_| Msg::SubjectChanged);
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

                <h1>
                    {format!("Name: {}", self.name.clone())}<br/>
                    {format!("Email: {}", self.email.clone())}<br/>
                    {format!("Subject: {}", self.subject.clone())}<br/>
                    {format!("Message: {}", self.message.clone())}<br/>
                    {format!("Out: {}", self.out.clone())}<br/>
                </h1>

                <div class="contact-form">
                    <label for="name">{"Name"}</label>
                    <input ref={self.name_input.clone()} onchange={name_onchange} class="form-control" type="text" id="name" name="name" placeholder="Your name..." />
                    <label for="email">{"Email address"}</label>
                    <input ref={self.email_input.clone()} onchange={email_onchange} type="email" class="form-control" id="email" aria-describedby="emailHelp" placeholder="Enter email" />
                    <label for="subject">{"Subject"}</label>
                    <input ref={self.subject_input.clone()} onchange={subject_onchange} type="text" class="form-control" id="subject" placeholder="Enter subject" />
                    <label for="message">{"Subject"}</label>
                    <textarea ref={self.message_input.clone()} onchange={message_onchange} class="form-control" id="subject" name="message" placeholder="Write something.." style="height:200px"></textarea>
                    <button class="btn btn-primary btn-rounded btn-sm nav-button selectDisable" onclick={ctx.link().callback(|_| Msg::SubmitForm)}>{"Submit"}</button>
                </div>
            </div>
        }
    }
}