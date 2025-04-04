use yew::{Component, Context, Html};
const HTML: &str = include_str!("../components/talks/talks.html");


pub struct Talks;

impl Component for Talks{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Html::from_html_unchecked(HTML.into())
    }
}
