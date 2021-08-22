use yew::prelude::*;

use graphql::Quiz;
use pbs::prelude::*;
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
        let Props { players, quiz, .. } = &self.props;

        let body = html! { <Scores players={players.clone()}/> };

        html! {
            <>
                <cbs::TitleHero color={Color::Primary} title={quiz.name.clone()} subtitle={"finished"}/>
                <Hero body={body} color={Color::Primary} size={HeroSize::Medium}/>
            </>
        }
    }
}
