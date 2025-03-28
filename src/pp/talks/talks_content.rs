use yew::{Component, Context, Html};

const HTML: &str = include_str!("talks.html");

pub struct TalksContent;

impl Component for TalksContent{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Html::from_html_unchecked(HTML.into())
    }
}