use yew::prelude::*;
use crate::routes::Route; // Import your Route enum
use crate::routes::RouterLink; // Import your RouterLink component

#[function_component(MainNav)]
pub fn main_nav() -> Html {
    html! {
        <nav class="main-navigation">
            <input type="checkbox" id="nav-toggle" class="nav-toggle" />
            <label for="nav-toggle" class="nav-toggle-label">
                <span></span>
            </label>
            
            <div class="nav-menu">
                <RouterLink to={Route::Home} class="nav-link">{"Home"}</RouterLink>
                <RouterLink to={Route::Talks} class="nav-link">{"Talks"}</RouterLink>
                <RouterLink to={Route::YoutubeGrid} class="nav-link">{"Youtube"}</RouterLink>
                <RouterLink to={Route::Person} class="nav-link">{"Me"}</RouterLink>
                <RouterLink to={Route::Goals} class="nav-link">{"Goals"}</RouterLink>
            </div>
        </nav>
    }
}
