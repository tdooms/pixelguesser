use std::rc::Rc;

use cobul::*;
use yew::*;
use yew_router::prelude::*;

use api::{Action, Participant, Phase, Quiz, Session, Stage};
use shared::{callback, use_toast, Route};

use crate::edit::PlayerEdit;
use crate::info::RoundInfo;
use crate::list::PlayerList;
use crate::navigate::Navigate;
use crate::rating::Rating;

mod edit;
mod info;
mod list;
mod navigate;
mod rating;

#[derive(Properties, Clone, PartialEq)]
pub struct PlayerProps {
    pub model: Model<String>,
    pub submit: Callback<()>,
}

#[function_component(PlayerForm)]
pub fn player_form(props: &PlayerProps) -> Html {
    let PlayerProps { model, submit } = props.clone();

    html! {
        <Columns>
        <Column>
        <Input {model} size={Size::Large} placeholder={"eg. Alex"} />
        </Column>

        <Column size={ColumnSize::IsNarrow}>
        <simple::Field enter={submit.clone()}>
            <simple::Button click={submit} size={Size::Large} color={Color::Info} icon={fa::Solid::Plus} />
        </simple::Field>
        </Column>
        </Columns>
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub session: Rc<Session>,
    pub quiz: Rc<Quiz>,

    pub action: Callback<Action>,
}

#[function_component(Manage)]
pub fn manage(props: &Props) -> Html {
    let Props { session, quiz, action } = props.clone();

    let toast = use_toast();

    let name = use_state(|| String::default());
    let model = Model { value: (*name).clone(), input: callback!(name; move |new| name.set(new)) };

    let rounds = quiz.rounds.len();

    if !session.participants.contains_key(&Participant::Host) {
        toast.warning("Host left the session", true);
    }

    let remove = action.reform(Action::Remove);

    let submit = callback!(name, action; move |_| {
        action.emit(Action::Add((*name).clone()));
        name.set(String::default());
    });
    let guess = |round: usize, quiz: &Quiz| {
        let points = quiz.rounds[round].points as i64;
        action.reform(move |name| Action::Score(name, points))
    };
    let leave = {
        let navigator = use_navigator().unwrap().clone();
        Callback::from(move |_| navigator.push(&Route::Overview))
    };

    let body = match session.phase {
        Phase::Playing { round, .. } if round >= rounds => html! {},
        Phase::Lobby => html! {
            <>
            <PlayerForm {model} {submit}/>
            <Block/>
            <PlayerList title={"Select a player to remove them."} session={session.clone()} click={remove}/>
            <Navigate click={action} {session} rounds={quiz.rounds.len()} />
            </>
        },
        Phase::Playing { stage: Stage::Info, round } => html! {
            <RoundInfo index={round} rounds={quiz.rounds.len()} round={quiz.rounds[round].clone()} />
        },
        Phase::Playing { stage: Stage::Running | Stage::Paused, round } => html! {
            <>
            <RoundInfo index={round} rounds={quiz.rounds.len()} round={quiz.rounds[round].clone()}/>
            <PlayerList title="Select the player who guessed correctly." session={session.clone()} click={guess(round, &quiz)}/>
            <Navigate click={action} {session} rounds={quiz.rounds.len()} />
            </>
        },
        Phase::Playing { stage: Stage::Revealing, .. } => html! {
            <Hero color={Color::Info}> <Title> {"Revealing ..."} </Title> </Hero>
        },
        Phase::Playing { stage: Stage::Scores, .. } => html! {
            <>
            <Hero color={Color::Info}> <Title> {"Showing scores"} </Title> </Hero>
            <Navigate click={action} session={session.clone()} rounds={quiz.rounds.len()} />
            </>
        },
        Phase::Playing { stage: Stage::Editing, .. } => html! {
            <>
            <PlayerEdit session={session.clone()} title="Edit players' score." submit={Callback::noop()} />
            <Navigate click={action} session={session.clone()} rounds={quiz.rounds.len()} />
            </>
        },
        Phase::Playing { round, stage: Stage::Revealed } => html! {
            <>
            <Hero color={Color::Info}> <Title> {format!("End of round {}", round + 1)} </Title> </Hero>
            <Navigate session={session.clone()} rounds={quiz.rounds.len()} click={action}/>
            </>
        },
        Phase::Finished => html! {
            <>
            <Rating {quiz} />
            <Buttons alignment={Alignment::Centered}>
            <simple::Button color={Color::Info} light=true click={leave} size={Size::Large} icon={fa::Solid::RightFromBracket} text="Leave"/>
            </Buttons>
            </>
        },
    };

    html! { <Section> <Container> { body } </Container> </Section> }
}
