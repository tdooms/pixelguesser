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

    let btn = |action: Action, color: Color, light: bool, icon: Solid, text: &str, hidden: bool| {
        html! {
            <Button size={Size::Large} onclick={callback.reform(move |_| action.clone())} {color} {light} {hidden}>
            <Icon {icon}/> <span>{text}</span>
            </Button>
        }
    };

    let buttons = |idx: &[usize]| {
        html! {
            <>
            {btn(Action::Start, Color::Primary, false, Solid::Play, "Start", !idx.contains(&0))}
            {btn(Action::Stage(Stage::Running), Color::Light, false, Solid::Play, "Resume", !idx.contains(&1))}
            {btn(Action::Stage(Stage::Paused), Color::Light, false, Solid::Pause, "Pause", !idx.contains(&2))}
            {btn(Action::Stage(Stage::Revealing), Color::Danger, true, Solid::Eye, "Reveal", !idx.contains(&3))}
            {btn(Action::Stage(Stage::Scores), Color::Info, true, Solid::ListOl, "Scores", !idx.contains(&4))}
            {btn(Action::Next, Color::Success, true, Solid::Forward, "Next", !idx.contains(&5))}
            {btn(Action::Finish, Color::Primary, true, Solid::FlagCheckered, "Final Scores", !idx.contains(&6))}
            </>
        }
    };

    let buttons = match session.phase {
        Phase::Playing { stage: Stage::Revealing | Stage::Revealed, round }
            if round >= rounds - 1 =>
        {
            buttons(&[6])
        }
        Phase::Lobby => buttons(&[0]),
        Phase::Playing { stage: Stage::Paused, .. } => buttons(&[1, 3]),
        Phase::Playing { stage: Stage::Running, .. } => buttons(&[2, 3]),
        Phase::Playing { stage: Stage::Revealing | Stage::Revealed, .. } => buttons(&[4]),
        Phase::Playing { stage: Stage::Scores, .. } => buttons(&[5]),
        _ => html! {},
    };

    html! {
        <Buttons alignment={Alignment::Centered} class="mt-4">
            { buttons }
        </Buttons>
    }
}
