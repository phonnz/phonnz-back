use yew::prelude::*;

#[function_component(MainNav)]
pub fn main_nav() -> Html {
    html! {
        <nav class="main-navigation">
            <input type="checkbox" id="nav-toggle" class="nav-toggle" />
            <label for="nav-toggle" class="nav-toggle-label">
                <span></span>
            </label>
            
            <div class="nav-menu">
                <a href="/" class="nav-link">{"Home"}</a>
                <a href="/about" class="nav-link">{"About"}</a>
                <a href="/services" class="nav-link">{"Services"}</a>
                <a href="/contact" class="nav-link">{"Contact"}</a>
            </div>
        </nav>
    }
}

