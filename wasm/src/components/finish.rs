use crate::components::Scores;
use api::{Player, Quiz};
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub session_id: u64,
    pub players: HashMap<u64, Player>,
    pub quiz: Quiz,
}

pub struct Finish {
    props: Props,
}

impl Component for Finish {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section class="hero">
                    <div class="hero-body">
                        <p class="title"> {&self.props.quiz.name} </p>
                        <p class="subtitle"> {"finished"} </p>
                    </div>
                </section>

                <section class="hero is-primary is-medium">
                    <div class="hero-body">
                        <Scores session_id=self.props.session_id players=self.props.players.clone() />
                    </div>
                </section>
            </>
        }
    }
}
