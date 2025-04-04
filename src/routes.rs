use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{HomeContent, Person, Talks, Goals, YoutubeGrid};


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
    #[at("/person")]
    Person,
    #[at("/talks")]
    Talks,
    #[at("/goals")]
    Goals,
    #[at("/youtube")]
    YoutubeGrid,
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

    let path = match props.to {
        Route::Home => "/",
        Route::Person => "/person",
        Route::Talks => "/talks",
        Route::Goals => "/goals",
        Route::YoutubeGrid=> "/youtube",
    };
    html! {
        <a 
            href={path} 
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
            <HomeContent />
        },
        Route::Person => html! {
            <Person />

        },Route::Talks => html! {
            <Talks />
        },Route::Goals => html! {
            <Goals />
        },
        Route::YoutubeGrid => html! {
            <YoutubeGrid />

        },
    }
}
