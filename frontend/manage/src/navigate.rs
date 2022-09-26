use std::rc::Rc;
use yew::*;

use api::{Action, Phase, Session, Stage};
use cobul::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub click: Callback<Action>,
    pub session: Rc<Session>,
    pub rounds: usize,
}

#[function_component(Navigate)]
pub fn navigate(props: &Props) -> Html {
    let Props { click, session, rounds } = props;
    let cb = |action: Action| click.reform(move |_| action.clone());

    let buttons = match session.phase {
        Phase::Playing { stage: Stage::Revealing | Stage::Revealed, round }
            if round >= rounds - 1 =>
        {
            html! { <simple::Button click={cb(Action::Finish)} size={Size::Large} color={Color::Primary} light=true icon={fa::Solid::FlagCheckered} text="Final Scores" /> }
        }
        Phase::Lobby => {
            html! { <simple::Button click={cb(Action::Start)} size={Size::Large} color={Color::Primary} icon={fa::Solid::Play} text="Start" /> }
        }
        Phase::Playing { stage: Stage::Revealing | Stage::Revealed, .. } => {
            html! { <simple::Button click={cb(Action::Stage(Stage::Scores))} size={Size::Large} color={Color::Info} icon={fa::Solid::ListOl} text="Scores" /> }
        }
        Phase::Playing { stage: Stage::Scores, .. } => {
            html! { <simple::Button click={cb(Action::Next)} size={Size::Large} color={Color::Success} light=true icon={fa::Solid::Forward} text="Next" /> }
        }
        Phase::Playing { stage: Stage::Paused, .. } => html! {
            <>
            <simple::Button click={cb(Action::Stage(Stage::Running))} size={Size::Large} color={Color::Light} icon={fa::Solid::Play} text="Resume" />
            <simple::Button click={cb(Action::Stage(Stage::Revealing))} size={Size::Large} color={Color::Danger} light=true icon={fa::Solid::Eye} text="Reveal" />
            </>
        },
        Phase::Playing { stage: Stage::Running, .. } => html! {
            <>
            <simple::Button click={cb(Action::Stage(Stage::Paused))} size={Size::Large} color={Color::Danger} icon={fa::Solid::Pause} text="Pause" />
            <simple::Button click={cb(Action::Stage(Stage::Revealing))} size={Size::Large} color={Color::Danger} light=true icon={fa::Solid::Eye} text="Reveal" />
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
