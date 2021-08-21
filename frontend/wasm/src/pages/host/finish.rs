use std::collections::HashMap;

use yew::prelude::*;

use graphql::Quiz;
use pbs::{Color, HeroSize};
use shared::Player;

use crate::pages::host::Scores;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub players: Vec<Player>,
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
        let body = html! { <Scores players={self.props.players.clone()}/> };

        html! {
            <>
                <cbs::TitleHero color={Color::Primary} title={self.props.quiz.name.clone()} subtitle={"finished"}/>
                <pbs::Hero body={body} color={Color::Primary} size={HeroSize::Medium}/>
            </>
        }
    }
}
