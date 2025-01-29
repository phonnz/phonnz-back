use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{Route, switch};

mod routes;
mod algos;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
