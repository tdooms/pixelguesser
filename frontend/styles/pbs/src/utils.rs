use yew::prelude::*;

pub fn enclose(class: impl Into<String>, option: Option<Html>) -> Html {
    html! { <div class={class.into()}> {option.unwrap_or_default()} </div> }
}

pub fn enclose_with_tag(tag: impl Into<String>, class: impl Into<String>, option: Option<Html>) -> Html {
    html! { <@{tag.into()} class={class.into()}> {option.unwrap_or_default()} </@> }
}