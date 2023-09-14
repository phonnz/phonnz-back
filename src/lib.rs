use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
// use yew::prelude::*;

#[http_component]
fn handle_phonnz_back(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
        Ok(http::Response::builder() 
        .status(200) 
        .header("foo", "bar") 
        .body(
        //.body(yew::Renderer::<App>::new().render().into()
Some("{'some': 'Value'}".into())
        )?) 
    //ok(yew::Renderer::<App>::new().render())
}



