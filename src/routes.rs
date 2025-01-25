use yew::prelude::*;
use yew_router::prelude::*;

mod home;
mod base_template;

use home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <base_template::BaseTemplate>
                <Home />
            </base_template::BaseTemplate>
        },

    }
}
