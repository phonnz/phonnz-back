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
                <Link<Route> to={Route::Talks}>{"Talks"}</Link<Route>>
                <Link<Route> to={Route::Youtube}>{"Youtube"}</Link<Route>>
                <Link<Route> to={Route::Person}>{"Person"}</Link<Route>>
                <Link<Route> to={Route::Goals}>{ "Goals 2025" }</Link<Route>>
        </nav>
            </header>
            <main>
                { for props.children.iter() }
            </main>
<footer>
      <div id="social">
           <a target="_blank" href="https://github.com/phonnz"><i class="bx bxl-github" ></i></a>
           <a target="_blank" href="https://linkedin.com/in/phonnz"><i class="bx bxl-linkedin" ></i></a>
           <a target="_blank" href="mailto:alfonso@intuitivo.com"><i class="bx bxl-gmail" ></i></a>
           <a target="_blank" href="https://twitter.com/phonnz"><i class="bx bxl-twitter" ></i></a>
           <a target="_blank" href="https://www.youtube.com/@fonscodifica"><i class="bx bxl-youtube"></i></a> 
            <span class="animate" style="--i:6;"></span>
        </div>
        <p>{"Built with 🦀, 🧉, "}<a target="_blank" href="https://yew.rs/">{"Yew"}</a>{" & "}<a target="_blank" href="https://www.fermyon.com/spin" >{"Spin"}</a>{" &copy; 2025 "}</p>
        <p></p>
    </footer>
        </div>
    }
} 
