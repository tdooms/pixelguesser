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

    let button = |action: Action,
                  color: Color,
                  light: bool,
                  icon: Icons,
                  text: &str,
                  hidden: bool| {
        html! {
            <Button size={Size::Large} onclick={callback.reform(move |_| action.clone())} {color} {light} {hidden}>
            <Icon {icon}/> <span>{text}</span>
            </Button>
        }
    };

    let buttons = |idx: &[usize]| {
        html! {
            <>
            {button(Action::Start, Color::Primary, false, Icons::Play, "start", !idx.contains(&0))}
            {button(Action::Stage(Stage::Running), Color::Light, false, Icons::Play, "resume", !idx.contains(&1))}
            {button(Action::Stage(Stage::Paused), Color::Light, false, Icons::Pause, "pause", !idx.contains(&2))}
            {button(Action::Stage(Stage::Revealing), Color::Danger, true, Icons::EyeSolid, "reveal", !idx.contains(&3))}
            {button(Action::Stage(Stage::Scores), Color::Info, true, Icons::ListOl, "scores", !idx.contains(&4))}
            {button(Action::Next, Color::Success, true, Icons::Forward, "next", !idx.contains(&5))}
            {button(Action::Finish, Color::Primary, true, Icons::FlagCheckered, "finish", !idx.contains(&6))}
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
