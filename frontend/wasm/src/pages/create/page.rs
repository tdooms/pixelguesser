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
}

enum Stage {
    Quiz,
    Rounds,
    Confirm,
}

#[derive(Properties, Clone)]
pub struct Props {
    draft_id: Option<u64>,
}

pub struct Create {
    props: Props,
    link: ComponentLink<Self>,
    stage: Stage,
}

impl Component for Create {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link, stage: Stage::Quiz }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
            Msg::Done => self.stage = Stage::Confirm
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.stage {
            Stage::Quiz => html! { <CreateQuiz oncontinue={self.link.callback(Msg::Continue)} oncancel={self.link.callback(|_| Msg::Cancel)}/> },
            Stage::Rounds => html! { <CreateRounds ondone={self.link.callback(|_| Msg::Done)} onback={self.link.callback(|_| Msg::Back)}/> },
            Stage::Confirm => html! { <Confirm onconfirm={self.link.callback(|_| Msg::Confirm)}/>}
        }
    }
}
