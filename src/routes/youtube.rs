use yew::prelude::*;
mod youtube_grid;
use youtube_grid::YoutubeGrid;

#[function_component(Youtube)]
pub fn youtube() -> Html {
    html! {
        <YoutubeGrid/>
    }
}