#[macro_use]
extern crate yew;

use yew::prelude::*;

struct AppModel {
    value: i64,
}

enum Msg {
    Increment,
    Decrement,
}

impl Component for AppModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        AppModel {
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

impl Renderable<AppModel> for AppModel {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <header class="Banner",>
                    <h1 class="PageTitle",>
                        { "Welcome to George's website" }
                    </h1>
                </header>
                <div class="PageBody",>
                    <button onclick=|_| Msg::Increment,>
                        { "Add 1" }
                    </button>
                    <button onclick=|_| Msg::Decrement,>
                        { "Subtract 1" }
                    </button>
                    <p>{ self.value }</p>
                </div>
                <footer class="Footer",>
                    { "The source for this site is available " }
                    <a href="https://github.com/g-s-k/gsk-space",>{ "here" }</a>
                    { "." }
                </footer>
            </>
        }
    }
}

fn main() {
    yew::initialize();
    App::<AppModel>::new().mount_to_body();
    yew::run_loop();
}
