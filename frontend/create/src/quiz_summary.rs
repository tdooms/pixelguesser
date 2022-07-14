use cobul::*;
use std::rc::Rc;
use yew::*;

use api::{DraftQuiz, DraftRound, Resolution};

use crate::Stage;
use ywt::callback;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onstage: Callback<Stage>,
    pub rounds: Rc<Vec<DraftRound>>,
    pub quiz: Rc<DraftQuiz>,
}

#[function_component(Summary)]
pub fn summary(props: &Props) -> Html {
    let Props { onstage, quiz, rounds } = props;
    let ondone = callback!(onstage; move |_| onstage.emit(Stage::Done));
    let onback = callback!(onstage; move |_| onstage.emit(Stage::Rounds));

    let round_mapper = |round: &DraftRound| {
        html! {
            <Column size={ColumnSize::Is3}>
            <custom::DynImage src={round.image.src(Resolution::Thumbnail)} height=20/>
            <p class="has-text-centered"> <b>{round.answer.clone()}</b> </p>
            </Column>
        }
    };

    html! {
        <Section>
        <Container>

        <Hero color={Color::Primary}>
            <Title> {&quiz.title} </Title>
            <Subtitle> {&quiz.description} </Subtitle>
        </Hero>

        <Box class="mt-5">
        <Columns multiline=true>
            { for rounds.iter().map(round_mapper) }
        </Columns>
        </Box>

        <Buttons>
            <Button color={Color::Info} light=true onclick={onback}>
                <span> {"Rounds"} </span>
            </Button>
            <Button color={Color::Info} onclick={ondone}>
                <span> {"Submit"} </span>
            </Button>
        </Buttons>

        </Container>
        </Section>
    }
}
