use yew::prelude::*;

use crate::properties::{Color, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    children: Children,

    #[prop_or_default]
    header: Option<Html>,

    #[prop_or_default]
    ondelete: Option<Callback<()>>,

    #[prop_or_default]
    color: Option<Color>,

    #[prop_or_default]
    size: Size,
}

#[function_component(Message)]
pub fn message(props: &Props) -> Html {
    let header = match (props.header.clone(), props.ondelete.clone()) {
        (Some(html), Some(ondelete)) => html! {
            <div class="message-header"> {html}
                <button class="delete" aria-label="delete" onclick={ondelete.reform(|_| ())}></button>
            </div>
        },
        (None, Some(ondelete)) => html! {
            <div class="message-header">
                <button class="delete" aria-label="delete" onclick={ondelete.reform(|_| ())}></button>
            </div>
        },
        (Some(html), None) => html! {<div class="message-header"> {html} </div> },
        (None, None) => html! {}
    };

    html! {
        <article class={classes!("message", props.color, props.size)}>
            { header }
            <div class="message-body">
                { for props.children.iter() }
            </div>
        </article>
    }
}