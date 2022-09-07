use std::rc::Rc;

use api::{Action, Participant, Phase, Quiz, Session, Stage};
use cobul::*;
use shared::{use_toast, Route};
use yew::*;
use yew_router::prelude::*;

use crate::info::RoundInfo;
use crate::list::PlayerList;
use crate::navigate::Navigate;
use crate::player_form::PlayerForm;
use crate::rating::Rating;

mod info;
mod list;
mod navigate;
mod rating;

#[function_component(NameField)]
pub fn name_field(props: &honey::CustomProps<Player>) -> Html {
    // honey::CustomProps { submit, input, value, .. } = props.clone();

    html! {
        <Field grouped=true>
            <Control expanded=true>
                <Input {input} size={Size::Large} r#type={InputType::Text} placeholder={"eg. Alex"} {value}/>
            </Control>
            <Control>
                <Button click={submit} size={Size::Large} color={Color::Info}>
                    <Icon icon={fa::Solid::Plus}> </Icon>
                </Button>
            </Control>
        </Field>
    }
}

#[derive(honey::Form, Default, Clone)]
#[form(enter)]
pub struct Player {
    #[form(custom = "name_field")]
    pub name: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub session: Rc<Session>,
    pub quiz: Rc<Quiz>,

    pub action: Callback<Action>,
}

#[function_component(Manage)]
pub fn manage(props: &Props) -> Html {
    let Props { session, quiz, action } = props;

    let toast = use_toast();
    let player = use_state(|| Player::default());

    if !session.participants.contains_key(&Participant::Host) {
        toast.warning("Host left the session", true);
    }

    let remove = callback.reform(Action::Remove);
    let input = ywt::callback!(player; move |new| player.set(new));

    let submit = ywt::callback!(player; move |_| {
        action.emit(Action::Add(player.name.clone()));
        player.set(Player::default());
    });
    let guess = |round: usize, quiz: &Quiz| {
        let points = quiz.rounds[round].points as i64;
        callback.reform(move |name| Action::Score(name, points))
    };

    let leave = {
        let navigator = use_navigator().unwrap().clone();
        Callback::from(move |_| navigator.push(Route::Overview))
    };

    let rounds = quiz.rounds.len();
    let body = match session.phase {
        Phase::Playing { round, .. } if round >= rounds => {
            html! {}
        }
        Phase::Lobby => html! {
            <>
            <PlayerForm value={player} {input} {submit}/>
            <Block/>
            <PlayerList title={"Select a player to remove them."} {session} onclick={onremove}/>
            <Navigate {session} rounds={quiz.rounds.len()} {callback}/>
            </>
        },
        Phase::Playing { stage: Stage::Info, round } => html! {
            <RoundInfo index={round} rounds={quiz.rounds.len()} round={quiz.rounds[round].clone()} />
        },
        Phase::Playing { stage: Stage::Running | Stage::Paused, round } => html! {
            <>
            <RoundInfo index={round} rounds={quiz.rounds.len()} round={quiz.rounds[round].clone()}/>
            <PlayerList title={"Select the player who guessed correctly."} {session} onclick={onguess(round, &quiz)}/>
            <Navigate session={session.clone()} rounds={quiz.rounds.len()} {callback}/>
            </>
        },
        Phase::Playing { stage: Stage::Revealing, .. } => html! {
            <Hero color={Color::Primary}> <Title> {"Revealing ..."} </Title> </Hero>
        },
        Phase::Playing { stage: Stage::Scores, .. } => html! {
            <>
            <Hero color={Color::Primary}> <Title> {"Showing scores"} </Title> </Hero>
            <Navigate session={session.clone()} rounds={quiz.rounds.len()} {callback}/>
            </>
        },
        Phase::Playing { round, stage: Stage::Revealed } => html! {
            <>
            <Hero color={Color::Primary}>
                <Title> {format!("End of round {}", round + 1)} </Title>
            </Hero>
            <Navigate session={session.clone()} rounds={quiz.rounds.len()} {callback}/>
            </>
        },
        Phase::Finished => html! {
            <>
            <Rating {quiz} />
            <Buttons alignment={Alignment::Centered}>
                <Button color={Color::Primary} light=true onclick={onleave} size={Size::Large}>
                    <Icon icon={fa::Solid::RightFromBracket}/> <span> {"leave"} </span>
                </Button>
            </Buttons>
            </>
        },
    };

    html! { <Section> <Container> { body } </Container> </Section> }
}
