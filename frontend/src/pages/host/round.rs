use super::Ranking;
use crate::components::Pixelate;
use crate::graphql::Quiz;
use gloo::timers::callback::Timeout;
use sessions::Player;
use std::collections::HashMap;
use yew::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub round: usize,
    pub rounds: usize,

    pub paused: bool,
    pub revealing: bool,

    pub url: String,
    pub players: HashMap<String, Player>,
}

pub enum Msg {
    Timer,
    Revealed,
}

pub struct RoundComponent {
    timer: Option<Timeout>,
    round: usize,
}

impl Component for RoundComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        Self {
            timer: Some(Timeout::new(5_000, move || link.send_message(Msg::Timer))),
            round: ctx.props().round,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Timer => self.timer = None,
            Msg::Revealed => {
                self.timer = Some(Timeout::new(2_000, move || link.send_message(Msg::Timer)))
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx.props().round != self.round {
            self.round = ctx.props().round;

            let link = ctx.link().clone();
            self.timer = Some(Timeout::new(5_000, move || link.send_message(Msg::Timer)));
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { round, rounds, paused, revealed: revealing, url, players } =
            ctx.props().clone();
        let onrevealed = ctx.link().callback(|_| Msg::Revealed);

        match (revealing, &self.timer) {
            (false, Some(_)) => html! {
                "starting round x/x"
            },
            (true, None) => html! {
                <Ranking players={players.clone()} />
            },
            _ => html! {
                <Pixelate revealing={revealed} {paused} {url} {onrevealed}/>
            },
        }
    }
}
