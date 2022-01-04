use strum::IntoEnumIterator;
use yew::prelude::*;

use cobul::props::{Alignment, Color, Size};
use cobul::*;
use sessions::{Action, Session};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub callback: Callback<Action>,
    pub session: Session,
    pub rounds: usize,
}

#[function_component(Navigate)]
pub fn navigate(props: &Props) -> Html {
    let Props { callback, session, rounds } = props;

    // button text, button icon, color, inverted
    let button_attrs = |action: &Action| match action {
        Action::Start => Some(("start", "fas fa-play", Color::Primary, false)),
        Action::Pause => Some(("pause", "fas fa-pause", Color::Light, false)),
        Action::Resume => Some(("resume", "fas fa-play", Color::Light, false)),
        Action::Reveal => Some(("reveal", "fas fa-eye", Color::Danger, true)),
        Action::Scores => Some(("scores", "fas fa-list-ol", Color::Link, true)),
        Action::Next => Some(("next", "fas fa-forward", Color::Success, false)),
        Action::Leave => Some(("leave", "fas fa-sign-out-alt", Color::Danger, true)),
        _ => None,
    };

    let actions = session.actions(*rounds);

    let button_style = |action: Action| {
        let (text, icon, color, light) = button_attrs(&action)?;
        let hidden = !actions.contains(&action);

        let html = html! {
            <Button hidden={hidden} color={color} light={light} size={Size::Large} onclick={callback.reform(move |_| action.clone())}>
                <Icon icon={icon}/> <span>{text}</span>
            </Button>
        };
        Some(html)
    };

    html! {
        <Buttons alignment={Alignment::Centered} extra="mt-4">
            { for Action::iter().filter_map(button_style) }
        </Buttons>
    }
}
