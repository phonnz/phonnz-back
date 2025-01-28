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
               <img src="assets/images/phonnz.png" class="logo" alt="phonnz" />
                <input class="side-menu" type="checkbox" id="side-menu"/>
                <label class="hamb" for="side-menu"><span class="hamb-line"></span></label>
                <nav class="nav">
                <Link<Route> to={Route::Home}>{ "Engineer" }</Link<Route>>
                <Link<Route> to={Route::Goals}>{ "Goals 2025" }</Link<Route>>
<ul class="menu">
                <li><a href="#talks">{"Talks"}</a></li> 
                <li><a href="#youtube">{"Youtube"}</a></li> 
                <li><a href="#person">{"Person"}</a> </li>
                <li><a href="#goals">{"Goals"}</a></li>
                </ul>
        </nav>
            </header>
            <main>
                { for props.children.iter() }
            </main>
        </div>
    }
} 
