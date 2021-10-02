use yates::canvas::*;
use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
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
            Msg::AddOne => {
                self.value += 1;

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Canvas width=800 height=800>
<circle cx="50" cy="50" r="40" stroke="green" stroke-width="4" fill="yellow" />
                </Canvas>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
