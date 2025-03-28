use yew::prelude::*;
use yew_router::prelude::*;

mod algos;
mod routes; 
mod components;
mod pages;

use routes::{Route, switch};
use components::MainNav;


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
        <MainNav />
        <main>
            <Switch<Route> render={switch} />
        </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
