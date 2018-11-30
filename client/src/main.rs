#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
    value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
            }
            Msg::Decrement => {
                self.value -= 1;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <button onclick=|_| Msg::Increment,>{ "Add 1" }</button>
                <button onclick=|_| Msg::Decrement,>{ "Subtract 1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
