
use yew::{Component, Context, Html};

const HTML: &str = include_str!("main.html");

pub struct MainContent;

impl Component for MainContent{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Html::from_html_unchecked(HTML.into())
    }
}