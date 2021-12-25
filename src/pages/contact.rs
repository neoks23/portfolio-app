use yew::prelude::*;
use yew::events::{InputEvent};
use stdweb::*;
use crate::app_router::{AppRoute};
use yew_router::prelude::*;


pub enum Msg{
    UpdateName(String),
    UpdateMail(String),
    UpdateSubject(String),
    UpdateMessage(String),
}

pub struct Contact {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    name: String,
    email: String,
    subject: String,
    message: String,
}

impl Component for Contact {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "".to_string(),
            email: "".to_string(),
            subject: "".to_string(),
            message: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg{
            Msg::UpdateName(name) => self.name += &name.clone(),
            Msg::UpdateMail(email) => self.email += &email.clone(),
            Msg::UpdateSubject(subject) => self.subject += &subject.clone(),
            Msg::UpdateMessage(message) => self.message += &message.clone(),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let history =  ctx.link().history().unwrap();
        let home_history = history.clone();
        let credits_history = history.clone();
        let home = Callback::once(move |_| home_history.push(AppRoute::Index));
        let credits = Callback::once(move |_| credits_history.push(AppRoute::Credits));

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
                </h1>
                <div class="contact-form">
                    <textarea class="form-control" id="exampleFormControlInput1" placeholder="Simon Sinek"
                        oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateName(event.data().unwrap()))}
                        value={self.name.clone()}>
                    </textarea>
                    <textarea type="email" class="form-control" id="exampleFormControlInput2" placeholder="name@example.com"
                        oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateMail(event.data().unwrap()))}
                        value={self.email.clone()}>
                    </textarea>
                    <textarea class="form-control" id="exampleFormControlInput3" placeholder="Insert subject"
                        oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateSubject(event.data().unwrap()))}
                        value={self.subject.clone()}>
                    </textarea>
                    <textarea class="form-control" id="exampleFormControlInput4" placeholder="Insert message"
                        oninput={ctx.link().callback(|event: InputEvent| Msg::UpdateMessage(event.data().unwrap()))}
                        value={self.message.clone()}>
                    </textarea>
                </div>
            </div>
        }
    }
}