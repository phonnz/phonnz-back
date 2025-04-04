use yew::prelude::*;
use yew_router::prelude::*;

mod algos;
mod routes; 
mod components;
mod pages;

use routes::{Route, switch};
use components::{MainNav, Footer};


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
        <MainNav />
        <main>
            <Switch<Route> render={switch} />
        </main>
        <Footer />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
