use yew::prelude::*;

use api::*;
use pbs::HeroSize;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub quiz: Quiz,
}

pub struct Rating {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Rating {
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
            <pbs::Subtitle> {self.props.quiz.name.clone()} </pbs::Subtitle>
        };

        let body = html! {
            <pbs::Container extra="has-text-centered">
                <pbs::Title> {"give rating"} </pbs::Title>
                <pbs::Subtitle> {"TODO"} </pbs::Subtitle>
            </pbs::Container>
        };

        html! {
            <pbs::Hero size={HeroSize::Medium} header={header} body={body}/>
        }
    }
}
