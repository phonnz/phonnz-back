use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{HomeContent, YoutubeGrid};


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
        Route::YoutubeGrid => html! {
            <YoutubeGrid />

        },
    }
}
