use yew::prelude::*;
use yewtil::NeqAssign;

use api::*;
use pbs::HeroSize;
use crate::utils::Round;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub rounds: usize,
    pub round: Round,
}

pub struct RoundInfo {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for RoundInfo {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let header = html! {
            <pbs::Subtitle> {"round "} {self.props.index} {" / "} {self.props.rounds} </pbs::Subtitle>
        };

        let body = html! {
            <pbs::Container extra="has-text-centered">
                <pbs::Title> {self.props.round.answer.clone()} </pbs::Title>
                <pbs::Subtitle> {self.props.round.points} {" points"} </pbs::Subtitle>
            </pbs::Container>
        };

        html! {
            <pbs::Hero size={HeroSize::Medium} header={header} body={body}/>
        }
    }
}
