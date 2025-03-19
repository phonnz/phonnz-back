use yew::{Component, Context, Html, Properties, html};

#[derive(Properties, PartialEq)]
pub struct VideoCardProps {
    pub url: String,
}

pub struct VideoCard;

impl Component for VideoCard {
    type Message = ();
    type Properties = VideoCardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="video-card">
                <iframe 
                width="100%"
                height="auto"
                    src={ctx.props().url.clone()}
                    title="YouTube video player"
                    frameborder="0"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                    allowfullscreen=true
                />
            </div>
        }
    }
} 