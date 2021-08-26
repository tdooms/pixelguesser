use yew::prelude::*;
use yew::web_sys;
use yew::web_sys::HtmlInputElement;

use crate::properties::{Alignment, Boxed, Color, Fullwidth, Size};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub fullwidth: Fullwidth,

    #[prop_or_default]
    pub filename: Option<String>,

    #[prop_or_default]
    pub accept: Option<String>,

    #[prop_or_default]
    pub boxed: Boxed,

    #[prop_or_default]
    pub alignment: Alignment,

    #[prop_or_default]
    pub extra: String,

    pub onupload: Callback<Vec<web_sys::File>>,
}

#[function_component(File)]
pub fn file(props: &Props) -> Html {
    let maybe_file = || match &props.filename {
        None => html! {},
        Some(file) => html! {<span class="file-name"> {file} </span>},
    };

    let classes = classes!(
        "file",
        props.filename.as_ref().map(|_| "has-name"),
        props.boxed,
        props.fullwidth,
        props.alignment,
        &props.extra
    );

    let onchange = props.onupload.reform(|e: Event| {
        let files = e.target_unchecked_into::<HtmlInputElement>().files().unwrap();
        (0..files.length()).filter_map(|i| files.get(i)).collect()
    });

    html! {
        <div class={classes}>
            <label class="file-label">
            <input class="file-input" type="file" accept={props.accept.clone()} onchange={onchange} />
            <span class="file-cta">
                <span class="file-icon">
                <i class="fas fa-upload"></i>
                </span>
                <span class="file-label">
                    {"Choose a file..."}
                </span>
            </span>
                { maybe_file() }
            </label>
        </div>
    }
}
