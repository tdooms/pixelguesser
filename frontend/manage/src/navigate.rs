use std::rc::Rc;
use yew::*;

use api::{Action, Phase, Session, Stage};
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

    let button = |action: Action, color: Color, light: bool, icon: fa::Solid, text: &str| {
        let click = callback.reform(move |_| action.clone());
        html! {<Button size={Size::Large} {click} {color} {light}><Icon {icon}/> <span>{text}</span> </Button> }
    };

    let buttons = match session.phase {
        Phase::Playing { stage: Stage::Revealing | Stage::Revealed, round }
            if round >= rounds - 1 =>
        {
            html! { button(Action::Finish, Color::Primary, true, fa::Solid::FlagCheckered, "Final Scores") }
        }
        Phase::Lobby => {
            html! { button(Action::Start, Color::Primary, false, fa::Solid::Play, "Start") }
        }
        Phase::Playing { stage: Stage::Revealing | Stage::Revealed, .. } => {
            html! {button(Action::Stage(Stage::Scores), Color::Info, true, fa::Solid::ListOl, "Scores")}
        }
        Phase::Playing { stage: Stage::Scores, .. } => {
            html! {button(Action::Next, Color::Success, true, fa::Solid::Forward, "Next")}
        }
        Phase::Playing { stage: Stage::Paused, .. } => html! {
            <>
            { button(Action::Stage(Stage::Running), Color::Light, false, fa::Solid::Play, "Resume") }
            { button(Action::Stage(Stage::Revealing), Color::Danger, true, fa::Solid::Eye, "Reveal") }
            </>
        },
        Phase::Playing { stage: Stage::Running, .. } => html! {
            <>
            { button(Action::Stage(Stage::Paused), Color::Light, false, fa::Solid::Pause, "Pause") }
            { button(Action::Stage(Stage::Revealing), Color::Danger, true, fa::Solid::Eye, "Reveal") }
            </>
        },
        _ => html! {},
    };

    html! {
        <Buttons alignment={Alignment::Centered} class="mt-4">
            { buttons }
        </Buttons>
    }
}
