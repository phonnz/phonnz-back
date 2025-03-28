use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{Route, switch};
use components::MainNav

mod routes;
mod algos;

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
