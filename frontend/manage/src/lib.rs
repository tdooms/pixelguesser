use std::rc::Rc;

use api::{Action, Participant, Phase, Quiz, Session, Stage};
use cobul::*;
use shared::{use_toast, Route};
use yew::*;
use yew_router::prelude::*;

use crate::info::RoundInfo;
use crate::list::PlayerList;
use crate::navigate::Navigate;
use crate::rating::Rating;

mod info;
mod list;
mod navigate;
mod rating;

#[derive(Default, Clone, PartialEq)]
pub struct Player {
    pub name: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct PlayerProps {
    pub value: String,
    pub input: Callback<String>,
    pub submit: Callback<()>,
}

#[function_component(PlayerForm)]
pub fn name_field(props: &PlayerProps) -> Html {
    let PlayerProps { value, input, submit } = props.clone();

    html! {
        <Field grouped=true>
            <Control expanded=true>
            <Input {input} {value} size={Size::Large} placeholder={"eg. Alex"} />
            </Control>
            <Control>
            <simple::Button click={submit} size={Size::Large} color={Color::Info} icon={fa::Solid::Plus} />
            </Control>
        </Field>
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
    let player = use_state(|| Player::default());
    let rounds = quiz.rounds.len();

    if !session.participants.contains_key(&Participant::Host) {
        toast.warning("Host left the session", true);
    }

    let remove = action.reform(Action::Remove);
    let input = ywt::callback!(player; move |new| player.set(Player{name: new}));

    let submit = ywt::callback!(player, action; move |_| {
        action.emit(Action::Add(player.name.clone()));
        player.set(Player::default());
    });
    let guess = |round: usize, quiz: &Quiz| {
        let points = quiz.rounds[round].points as i64;
        action.reform(move |name| Action::Score(name, points))
    };
    let leave = {
        let navigator = use_navigator().unwrap().clone();
        Callback::from(move |_| navigator.push(Route::Overview))
    };

    let body = match session.phase {
        Phase::Playing { round, .. } if round >= rounds => html! {},
        Phase::Lobby => html! {
            <>
            <PlayerForm value={player.name.clone()} {input} {submit}/>
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
            <PlayerList title={"Select the player who guessed correctly."} session={session.clone()} click={guess(round, &quiz)}/>
            <Navigate click={action} {session} rounds={quiz.rounds.len()} />
            </>
        },
        Phase::Playing { stage: Stage::Revealing, .. } => html! {
            <Hero color={Color::Primary}> <Title> {"Revealing ..."} </Title> </Hero>
        },
        Phase::Playing { stage: Stage::Scores, .. } => html! {
            <>
            <Hero color={Color::Primary}> <Title> {"Showing scores"} </Title> </Hero>
            <Navigate click={action} session={session.clone()} rounds={quiz.rounds.len()} />
            </>
        },
        Phase::Playing { round, stage: Stage::Revealed } => html! {
            <>
            <Hero color={Color::Primary}> <Title> {format!("End of round {}", round + 1)} </Title> </Hero>
            <Navigate session={session.clone()} rounds={quiz.rounds.len()} click={action}/>
            </>
        },
        Phase::Finished => html! {
            <>
            <Rating {quiz} />
            <Buttons alignment={Alignment::Centered}>
            <simple::Button color={Color::Primary} light=true click={leave} size={Size::Large} icon={fa::Solid::RightFromBracket} text="Leave"/>
            </Buttons>
            </>
        },
    };

    html! { <Section> <Container> { body } </Container> </Section> }
}
