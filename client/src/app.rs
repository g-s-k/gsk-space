use yew::prelude::*;
use header::Header;
use footer::Footer;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <Header: title="Welcome to George's website", />
                <div class="PageBody",>
                </div>
                <Footer: />
            </>
        }
    }
}
