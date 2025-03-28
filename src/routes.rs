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

#[derive(Properties, PartialEq)]
pub struct RouterLinkProps {
    pub to: Route,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    Talks,
    #[at("/talks")]
    #[at("/youtube")]
    Youtube,
    #[at("/person")]
    Person,
    #[at("/goals")]
    Goals,
}


#[function_component(RouterLink)]
pub fn router_link(props: &RouterLinkProps) -> Html {
    let navigator = use_navigator().expect("Navigator not found");
    
    let onclick = {
        let navigator = navigator.clone();
        let to = props.to.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            navigator.push(&to);
        })
    };

    html! {
        <a 
            href={props.to.to_string()} 
            class={props.class.clone()} 
            onclick={onclick}
        >
            { props.children.clone() }
        </a>
    }
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
