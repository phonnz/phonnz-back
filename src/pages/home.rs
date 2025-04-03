
use yew::{Component, Context, Html};
const HTML: &str = include_str!("../components/home.html");

pub struct HomeContent;

impl Component for HomeContent{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Html::from_html_unchecked(HTML.into())
    }
}
