use cobul::{fa::Solid, Color, Column, Columns, Delete, Icon, Message, TextColor};
use shared::{Kind, Toast, UseToastManagerHandle};
use yew::*;

pub fn toast_view(toast: &dyn Toast, onremove: Callback<()>) -> Html {
    let (icon, color, text_color) = match toast.kind() {
        Kind::Error => (Solid::BoltLightning, Color::Danger, TextColor::Danger),
        Kind::Warning => (Solid::CircleExclamation, Color::Warning, TextColor::Warning),
        Kind::Info => (Solid::Info, Color::Info, TextColor::Info),
        Kind::Success => (Solid::Check, Color::Success, TextColor::Success),
    };

    html! {
        <Message {color}>
        <Columns>
        <Column> <Icon {icon} color={text_color}/> </Column>
        <Column> {toast.to_string()} </Column>
        <Column> <Delete onclick={onremove} /> </Column>
        </Columns>
        </Message>
    }
}

#[function_component(Toasts)]
pub fn toasts() -> Html {
    let toasts = use_context::<UseToastManagerHandle>().unwrap();
    let data = toasts.data();

    let onremove = move |id| Callback::from(move |_| toasts.remove(id));

    html! {
        <div style="position:absolute; top:55px; left:55px; z-index: 10">
        { for data.0.iter().map(move |(k, v)| toast_view(&*v.0, onremove.clone()(*k))) }
        </div>
    }
}
