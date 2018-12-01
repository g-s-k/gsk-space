use yew::prelude::*;

pub struct Header {
    pub title: String,
}

#[derive(Clone, Default, PartialEq)]
pub struct Props {
    pub title: String,
}

impl Component for Header {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header {
            title: props.title
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        true
    }
}

impl Renderable<Header> for Header {
    fn view(&self) -> Html<Self> {
        html!{
            <header class="Banner",>
                <h1 class="PageTitle",>
                    { &self.title }
                </h1>
            </header>
        }
    }
}
