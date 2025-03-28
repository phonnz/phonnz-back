use yew::prelude::*;

mod main_content;
use main_content::MainContent;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
            <>
        <MainContent/>
       </>
    }
}
