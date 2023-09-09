use yew::prelude::*;

pub struct NotFound;

impl Component for NotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        NotFound {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                {"Page Not Found"}
            </div>
        }
    }
}
