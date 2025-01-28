use yew::prelude::*;
mod goals;
use goals::GoalsContent;

#[function_component(Goals)]
pub fn goals() -> Html {
    html! {
            <>
        <GoalsContent/>
       </>
    }
}
