use yew::{Component, Context, Html};

const HTML: &str = include_str!("person.html");

pub struct PersonContent;

impl Component for PersonContent{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Html::from_html_unchecked(HTML.into())
    }
}