use cobul::*;
use std::rc::Rc;
use yew::*;

use api::FullQuiz;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub full: Rc<FullQuiz>,
}

#[function_component(Rating)]
pub fn rating(props: &Props) -> Html {
    html! {
        <Hero size={HeroSize::Medium}>
            <Container class="has-text-centered">
                <Title> {"give rating"} </Title>
                <Subtitle> {props.full.quiz.title.clone()} </Subtitle>
            </Container>
        </Hero>
    }
}
