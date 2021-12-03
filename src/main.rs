use yew::prelude::*;
use stdweb::*;

enum Msg {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.value += 2;
                js!{
                    var audio = new Audio("../../../assets/UI_Quirky7.mp3");
                    audio.cloneNode().play();
                };

                true
            },
            Msg::Subtract => {
                self.value -= 2;
                js!{
                    var audio = new Audio("../../../assets/UI_Quirky8.mp3");
                    audio.cloneNode().play();
                };

                true
            },
            Msg::Multiply => {
                self.value *= 2;
                js!{
                    var audio = new Audio("../../../assets/UI_Quirky9.mp3");
                    audio.cloneNode().play();
                };

                true
            },
            Msg::Divide =>{
                self.value /= 2;
                js!{
                    var audio = new Audio("../../../assets/UI_Quirky10.mp3");
                    audio.cloneNode().play();
                };

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
            <div class="centerdiv">
                <div class="menubox">
                    <div class="hexagon normalHexSize hexpos1" onclick=self.link.callback(|_| Msg::Add) >
                        <p class="noselect">{ format!("+ {} +", self.value)  }</p>
                    </div>
                    <div class="hexagon normalHexSize hexpos2" onclick=self.link.callback(|_| Msg::Subtract)>
                            <p class="noselect">{ format!("- {} -", self.value)  }</p>
                    </div>
                    <div class="hexagon normalHexSize hexpos3" onclick=self.link.callback(|_| Msg::Divide)>
                            <p class="noselect">{ format!("/ {} /", self.value)  }</p>
                    </div>
                    <div class="hexagon normalHexSize hexpos4" onclick=self.link.callback(|_| Msg::Multiply)>
                        <p class="noselect">{ format!("* {} *", self.value)  }</p>
                    </div>
                    <div class="hexagon normalHexSize hexpos5" onclick=self.link.callback(|_| Msg::Divide)>
                            <p class="noselect">{ format!("/ {} /", self.value)  }</p>
                    </div>
                    <div class="hexagon normalHexSize hexpos6" onclick=self.link.callback(|_| Msg::Subtract)>
                            <p class="noselect">{ format!("- {} -", self.value)  }</p>
                    </div>
                    <div class="hexagon normalHexSize hexpos7" onclick=self.link.callback(|_| Msg::Add)>
                            <p class="noselect">{ format!("+ {} +", self.value) }</p>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}