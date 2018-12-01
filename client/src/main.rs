#[macro_use]
extern crate yew;

mod header;

use yew::prelude::*;
use header::Header;

struct AppModel;

impl Component for AppModel {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        AppModel {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<AppModel> for AppModel {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <Header: title="Welcome to George's website", />
                <div class="PageBody",>
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
