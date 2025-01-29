use yew::prelude::*;
use yew_router::prelude::*;

mod base_template;
mod home;
mod goals;
mod person;
mod talks;
mod youtube;

use home::Home;
use goals::Goals;
use person::Person;
use talks::Talks;
use youtube::Youtube;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/talks")]
    Talks,
    #[at("/youtube")]
    Youtube,
    #[at("/person")]
    Person,
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
        Route::Talks => html! {
            <base_template::BaseTemplate>
                <Talks />
            </base_template::BaseTemplate>
        },
        Route::Youtube => html! {
            <base_template::BaseTemplate>
                <Youtube/>
            </base_template::BaseTemplate>
        },
        Route::Person => html! {
            <base_template::BaseTemplate>
                <Person/>
            </base_template::BaseTemplate>
        },
        Route::Goals => html! {
            <base_template::BaseTemplate>
                <Goals />
            </base_template::BaseTemplate>
        }
    }
}
