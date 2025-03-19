use yew::{Component, Context, Html , html};

const HTML: &str = include_str!("youtube.html");
const CSS: &str = include_str!("youtube.css");

pub struct YoutubeContent;

impl Component for YoutubeContent{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let style_html = Html::from_html_unchecked(format!("<style>{}</style>", CSS).into());
        
        let content_html = Html::from_html_unchecked(HTML.into());
        
        html! {
            <>
                {style_html}
                {content_html}
            </>
        }
    }
}