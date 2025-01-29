use yew::prelude::*;
mod youtube_content;
use youtube_content::YoutubeContent;

#[function_component(Youtube)]
pub fn youtube() -> Html {
    html! {
        <YoutubeContent/>
    }
}