use std::rc::Rc;

use gloo::timers::callback::Timeout;
use yew::*;

use sessions::Session;

use crate::components::Pixelate;
use crate::consts::{HOST_AFTER_REVEALED_TIME, HOST_ROUND_START_TIME};
use crate::graphql::FullQuiz;
use crate::utils::set_timer;

use super::{Ranking, RoundInfo};

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub index: usize,

    pub quiz: Rc<FullQuiz>,
    pub session: Rc<Session>,

    pub paused: bool,
    pub revealing: bool,
}

#[derive(Debug)]
pub enum Msg {
    Timer,
    Revealed,
}

#[derive(Debug, Clone, Copy)]
enum Stage {
    Info,
    Play,
    Show,
    Scores,
}

pub struct RoundPlay {
    timer: Option<Timeout>,
    index: usize,
    stage: Stage,
}

impl Component for RoundPlay {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            timer: Some(set_timer(ctx.link(), HOST_ROUND_START_TIME, Msg::Timer)),
            index: ctx.props().index,
            stage: Stage::Info,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.timer = None;

        match (msg, self.stage) {
            (Msg::Timer, Stage::Info) => self.stage = Stage::Play,
            (Msg::Timer, Stage::Show) => self.stage = Stage::Scores,
            (Msg::Revealed, Stage::Play) => {
                let link = ctx.link().clone();
                let callback = move || link.send_message(Msg::Timer);

                self.timer = Some(Timeout::new(HOST_AFTER_REVEALED_TIME, callback));
                self.stage = Stage::Show;
            }
            _ => return false,
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx.props().index != self.index {
            self.index = ctx.props().index;
            self.stage = Stage::Info;

            self.timer = Some(set_timer(ctx.link(), HOST_ROUND_START_TIME, Msg::Timer));
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { index, quiz, session, paused, revealing } = ctx.props().clone();
        let onrevealed = ctx.link().callback(|_| Msg::Revealed);
        let url = quiz.rounds[index].image.clone();

        match self.stage {
            Stage::Info => html! {
                <RoundInfo {index} rounds={quiz.rounds.len()} round={quiz.rounds[index].clone()}/>
            },
            Stage::Scores => html! {
                <Ranking {session} />
            },
            Stage::Play | Stage::Show => html! {
                <Pixelate {revealing} {paused} {url} {onrevealed}/>
            },
        }
    }
}
