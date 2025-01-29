use yew::prelude::*;
mod goals_content;
use goals_content::GoalsContent;

use crate::algos::basics;

#[function_component(Goals)]
pub fn goals() -> Html {
    basics::run();
    
    html! {
        <GoalsContent/>
    }
}
