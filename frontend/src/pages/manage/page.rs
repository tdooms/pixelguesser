use cobul::props::Color;
use cobul::*;
use yew::prelude::*;

use crate::graphql::{Quiz, Round};
use crate::route::Route;
use sessions::{Action, Session, Stage};
use yew_router::prelude::*;

use super::{Navigate, PlayerForm, PlayerList, PlayerMsg, PlayerName, Rating, RoundInfo};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub session: Session,
    pub quiz: Quiz,
    pub rounds: Vec<Round>,

    pub callback: Callback<Action>,
}

pub struct Manage {
    name: PlayerName,
}

pub enum Msg {
    PlayerForm(PlayerMsg)
}

impl Component for Manage {
    type Message = PlayerMsg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self { name: Default::default() }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PlayerForm(PlayerMsg::Change(name)) => {
                self.name = name;
            },
            Msg::PlayerForm(PlayerMsg::Submit(name)) => {
                let name = std::mem::take(&mut self.name.name)
                ctx.props().callback.emit(Action::AddPlayer(name.name));
                self.name
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let body = match session.stage {
            Stage::Lobby => {
                let title = "Select a player to remove them.";

                html! {
                <>
                    <PlayerForm inner={(*state).clone()} callback={form_callback}/>
                    <PlayerList title={title} players={session.players.clone()} onclick={onguess}/>
                </>
            }
            }
            Stage::Playing { round, paused: _ } => {
                let points = rounds[round].info.points as u64;
                let onguess = callback.reform(move |name| Action::Guessed(name, points));
                let title = "Select the player who guessed it.";

                html! {
                <>
                    <RoundInfo index={round} rounds={rounds.len()} round={rounds[round].clone()}/>
                    <PlayerList title={title} players={session.players.clone()} onclick={onguess}/>
                </>
            }
            }
            Stage::Revealed { round } => {
                html! { <Hero color={Color::Primary}><Title> {format!("End of round {}", round)} </Title> </Hero> }
            }
            Stage::Ranking { .. } => {
                html! { <Hero color={Color::Primary}><Title> {"Scores"} </Title> </Hero> }
            }
            Stage::Finished => {
                html! { <Rating quiz={props.quiz.clone()} />}
            }
            Stage::Left => {
                use_history().unwrap().push(Route::Overview);
                html! {}
            }
        };

        // TODO: this is probably a yew bug but the body appears under the buttons...
        html! {
            <Section>
                <Container>
                    { body }
                    <Navigate session={props.session.clone()} rounds={props.rounds.len()} callback={callback}/>
                </Container>
            </Section>
        }
    }
}

#[function_component(Manage)]
pub fn manage(props: &Props) -> Html {
    let Props { session, quiz: _, rounds, callback } = props;

    let state = use_state(|| PlayerName::default());

    let form_callback = {
        let state = state.clone();
        let cloned = callback.clone();

        Callback::from(move |msg| match msg {
            PlayerMsg::Submit(player) => {
                cloned.emit(Action::AddPlayer(player.name.clone()));
                state.set(player)
            }
            PlayerMsg::Change(player) => state.set(player),
        })
    };


}
