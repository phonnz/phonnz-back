use yew::prelude::*;
mod talks_content;
use talks_content::TalksContent;

#[function_component(Talks)]
pub fn talks() -> Html {
    html! {
        <TalksContent/>
    }
}