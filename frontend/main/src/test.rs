use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[hook]
fn use_name() -> SuspensionResult<String> {
    let (s, _handle) = Suspension::new();
    Err(s)
}

#[function_component(Content)]
fn content() -> HtmlResult {
    log::info!("rendering page");
    let name = use_name()?;
    Ok(html! {<div>{"Hello, "}{name}</div>})
}

#[function_component(Test)]
pub fn test() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};
    html! {<Suspense {fallback}> <Content /> </Suspense>}
}
