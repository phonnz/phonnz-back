use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{Route, RouterLink};

#[derive(Properties, PartialEq)]
pub struct NavProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(MainNav)]
pub fn main_nav(props: &NavProps) -> Html {
    let is_mobile_menu_open = use_state(|| false);

    let toggle_mobile_menu = {
        let is_mobile_menu_open = is_mobile_menu_open.clone();
        Callback::from(move |_| {
            is_mobile_menu_open.set(!*is_mobile_menu_open);
        })
    };

    // Close mobile menu when a link is clicked
    let close_mobile_menu = {
        let is_mobile_menu_open = is_mobile_menu_open.clone();
        Callback::from(move |_| {
            is_mobile_menu_open.set(false);
        })
    };

    html! {
        <nav class={classes!("main-navigation", props.class.clone())}>
            <div 
                class={classes!(
                    "nav-toggle-label", 
                    if *is_mobile_menu_open { "open" } else { "" }
                )}
                onclick={toggle_mobile_menu.clone()}
            >
                <span></span>
            </div>
            
            <div class={classes!(
                "nav-menu", 
                if *is_mobile_menu_open { "mobile-open" } else { "" }
            )}>
                <RouterLink 
                    to={Route::Home} 
                    class="nav-link"
                    onclick={close_mobile_menu.clone()}
                >
                    {"Home"}
                </RouterLink>
                <RouterLink 
                    to={Route::About} 
                    class="nav-link"
                    onclick={close_mobile_menu.clone()}
                >
                    {"About"}
                </RouterLink>
                <RouterLink 
                    to={Route::Services} 
                    class="nav-link"
                    onclick={close_mobile_menu.clone()}
                >
                    {"Services"}
                </RouterLink>
                <RouterLink 
                    to={Route::Contact} 
                    class="nav-link"
                    onclick={close_mobile_menu.clone()}
                >
                    {"Contact"}
                </RouterLink>
            </div>
        </nav>
    }
}
