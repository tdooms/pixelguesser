use api::{Image, Stage};
use components::Pixelate;
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
    let image = use_state(|| {
        Image::from_url("https://img.webmd.com/dtmcms/live/webmd/consumer_assets/site_images/article_thumbnails/other/cat_relaxing_on_patio_other/1800x1200_cat_relaxing_on_patio_other.jpg")
    });

    let stage = Stage::Running;
    let onreveal = Callback::noop();

    html! {<Pixelate image={(*image).clone()} {stage} {onreveal} />}
}
