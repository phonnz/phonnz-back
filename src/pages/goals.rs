use yew::{Component, Context, Html};

const HTML: &str = include_str!("../components/goals/goals.html");

pub struct Goals;

impl Component for Goals{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Html::from_html_unchecked(HTML.into())
    }
}
