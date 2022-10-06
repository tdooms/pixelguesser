use std::rc::Rc;

use cobul::*;
use yew::*;

use api::Quiz;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub quiz: Rc<Quiz>,
}

#[function_component(Rating)]
pub fn rating(props: &Props) -> Html {
    html! {
        <Hero size={HeroSize::Medium}>
            <Container class="has-text-centered">
                <Title> {"give rating"} </Title>
                <Subtitle> {props.quiz.title.clone()} </Subtitle>
            </Container>
        </Hero>
    }
}
