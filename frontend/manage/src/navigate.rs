use std::rc::Rc;
use strum::{EnumIter, IntoEnumIterator};
use yew::prelude::*;

use api::{Action, Session};
use cobul::props::{Alignment, Color, Size};
use cobul::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub callback: Callback<Action>,
    pub session: Rc<Session>,
    pub rounds: usize,
}

#[function_component(Navigate)]
pub fn navigate(props: &Props) -> Html {
    let Props { callback, session, rounds } = props;

    // button text, button icon, color, inverted
    let button_attrs = |action: &Action| match action {
        Action::StartQuiz => Some(("start", "fas fa-play", Color::Primary, false)),
        Action::PauseRound => Some(("pause", "fas fa-pause", Color::Light, false)),
        Action::ResumeRound => Some(("resume", "fas fa-play", Color::Light, false)),
        Action::RevealRound => Some(("reveal", "fas fa-eye", Color::Danger, true)),
        Action::NextRound => Some(("next", "fas fa-forward", Color::Success, false)),
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
        <Buttons alignment={Alignment::Centered} class="mt-4">
            { for Action::iter().filter_map(button_style) }
        </Buttons>
    }
}
