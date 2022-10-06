use std::rc::Rc;

use cobul::*;
use yew::*;

use api::{Action, Phase, Session, Stage};

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

    let finish = cb(Action::Finish);
    let start = cb(Action::Start);
    let next = cb(Action::Next);

    let scores = cb(Action::Stage(Stage::Scores));
    let edit = cb(Action::Stage(Stage::Editing));
    let run = cb(Action::Stage(Stage::Running));
    let pause = cb(Action::Stage(Stage::Paused));
    let reveal = cb(Action::Stage(Stage::Revealing));

    let buttons = match session.phase {
        Phase::Playing { stage: Stage::Revealing | Stage::Revealed, round }
            if round >= rounds - 1 =>
        {
            html! { <simple::Button click={finish} text="Podium" icon={fa::Solid::FlagCheckered} size={Size::Large} color={Color::Info} /> }
        }
        Phase::Lobby => {
            html! { <simple::Button click={start} text="Start" icon={fa::Solid::Play} size={Size::Large} color={Color::Info} /> }
        }
        Phase::Playing { stage: Stage::Revealing | Stage::Revealed, .. } => {
            html! { <simple::Button click={scores} text="Scores" icon={fa::Solid::ListOl} size={Size::Large} light=true color={Color::Info} /> }
        }
        Phase::Playing { stage: Stage::Editing, .. } => {
            html! { <simple::Button click={scores} text="Done" size={Size::Large} color={Color::Info} light=true icon={fa::Solid::Check}  /> }
        }
        Phase::Playing { stage: Stage::Scores, .. } => html! {
            <>
            <simple::Button click={edit} text="Edit" icon={fa::Solid::PenToSquare} size={Size::Large} color={Color::Info} light=true   />
            <simple::Button click={next} text="Next" icon={fa::Solid::Forward} size={Size::Large} color={Color::Info} />
            </>
        },
        Phase::Playing { stage: Stage::Paused, .. } => html! {
            <>
            <simple::Button click={run} text="Resume" icon={fa::Solid::Play} size={Size::Large} color={Color::Light}   />
            <simple::Button click={reveal} text="Reveal" icon={fa::Solid::Eye} size={Size::Large} color={Color::Info} light=true   />
            </>
        },
        Phase::Playing { stage: Stage::Running, .. } => html! {
            <>
            <simple::Button click={pause} size={Size::Large} color={Color::Light} icon={fa::Solid::Pause} text="Pause" />
            <simple::Button click={reveal} size={Size::Large} color={Color::Info} light=true icon={fa::Solid::Eye} text="Reveal" />
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
