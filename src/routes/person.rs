use yew::prelude::*;
mod person_content;
use person_content::PersonContent;

#[function_component(Person)]
pub fn person() -> Html {
    html! {
        <PersonContent/>
    }
}