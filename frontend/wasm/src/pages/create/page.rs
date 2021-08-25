use yew::prelude::*;

use crate::graphql::{DraftQuiz, DraftRound};
use crate::route::Route;

use super::{Confirm, CreateQuiz, CreateRounds};

pub enum Msg {
    Save(Vec<DraftRound>),
    Done,
    Confirm,
    Back,
    Continue(DraftQuiz),
    Cancel,
    Todo
}

enum Stage {
    Quiz,
    Rounds,
    Confirm,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    quiz_id: Option<u64>,
}

pub struct Create {
    stage: Stage,
}

impl Component for Create {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self { stage: Stage::Quiz }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Continue(quiz) => {
                // TODO: save quiz
                self.stage = Stage::Rounds;
            }
            Msg::Cancel => yew_router::push_route(Route::Overview),
            Msg::Confirm => {
                // TODO: change drafts to real quiz
                // TODO: leave page
            }
            Msg::Save(rounds) => {
                // TODO: save stuff
            }
            Msg::Back => self.stage = Stage::Quiz,
            Msg::Done => self.stage = Stage::Confirm,
            Msg::Todo => {}
        }
        true
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.stage {
            Stage::Quiz => html! { <CreateQuiz oncontinue={ctx.link().callback(Msg::Continue)} oncancel={ctx.link().callback(|_| Msg::Cancel)}/> },
            Stage::Rounds => html! { <CreateRounds ondone={ctx.link().callback(|_| Msg::Done)} onback={ctx.link().callback(|_| Msg::Back)}/> },
            Stage::Confirm => html! { <Confirm onback={ctx.link().callback(|_| Msg::Todo)} onconfirm={ctx.link().callback(|_| Msg::Confirm)}/>}
        }
    }
}
