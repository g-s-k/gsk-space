use yew::prelude::*;

pub struct Actions;

impl Component for Actions {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Actions
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Actions> for Actions {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="HashActions", />
        }
    }
}
