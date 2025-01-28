use yew::prelude::*;
use yew_router::prelude::*;

mod home;
mod goals;
mod base_template;

use home::Home;
use goals::Goals;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/goals")]
    Goals,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <base_template::BaseTemplate>
                <Home />
            </base_template::BaseTemplate>
        },
        Route::Goals => html! {
            <base_template::BaseTemplate>
                <Goals />
            </base_template::BaseTemplate>
        }
    }
}
