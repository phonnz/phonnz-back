use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
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
    </footer>    }
}

