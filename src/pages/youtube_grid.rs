use yew::{Component, Context, Html, html};
use crate::components::{VideoCard};
//mod crate::components::{youtube_card};

// const HTML: &str = include_str!("youtube.html");
const CSS: &str = include_str!("../components/video_card/youtube.css");

pub struct YoutubeGrid;

impl Component for YoutubeGrid{
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let video_urls = vec![
             "https://www.youtube.com/embed/hqNgxOp0i3g?si=6dxw5rAmKjW1_MHr".to_string(),
             "https://www.youtube.com/embed/RmZsUbJ_X0c?si=jcXuDlW7ZkF3MmNh".to_string(),
              "https://www.youtube.com/embed/D6iOsEoQ8Yk?si=04oJR9_5014qFgwl".to_string(),
             "https://www.youtube.com/embed/Sw4C4aU5f_4?si=L8m1cJBP_sf-I6oN".to_string(),
            
        ];

        let style_html = Html::from_html_unchecked(format!("<style>{}</style>", CSS).into());
       // let content_html = Html::from_html_unchecked(HTML.into());
        
        html! {
            <>
                {style_html}
            <section id="youtube">
            <h1>{"Spanish? Portugues? See what I'm learning in your language!"}</h1>
          <a href="https://www.youtube.com/@fonscodifica" target="_blank"><h3>{"Mira el canal completo!"}</h3></a>
                <div class="video-grid">
                    {
                        video_urls.iter().map(|url| {
                            html! {
                               <VideoCard url={url.clone()} />

                            }
                        }).collect::<Html>()
                    }
                </div>
                </section>
            </>
        }
    }
}
