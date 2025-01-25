use yew::prelude::*;
use yew_router::prelude::*;

use super::{Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(BaseTemplate)]
pub fn base_template(props: &Props) -> Html {
    html! {
        <div class="base-template">
            <header>
                <nav>
                    <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                </nav>
            </header>
            <main>
                { for props.children.iter() }
            </main>
        </div>
    }
} 
