use yew::prelude::*;
use yew::utils::NeqAssign;

use graphql::Quiz;
use pbs::prelude::*;

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
            <Subtitle> {self.props.quiz.name.clone()} </Subtitle>
        };

        let body = html! {
            <Container extra="has-text-centered">
                <Title> {"give rating"} </Title>
                <Subtitle> {"TODO"} </Subtitle>
            </Container>
        };

        html! {
            <Hero size={HeroSize::Medium} header={header} body={body}/>
        }
    }
}
