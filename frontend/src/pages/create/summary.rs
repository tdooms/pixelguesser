use yew::prelude::*;

use crate::graphql::{DraftQuiz, DraftRound, ImageData};
use cobul::props::{Color, ColumnSize};
use cobul::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub quiz: DraftQuiz,
    pub rounds: Vec<DraftRound>,

    pub onfinish: Callback<()>,
    pub onback: Callback<()>,
}

#[function_component(Summary)]
pub fn summary(props: &Props) -> Html {
    let Props { quiz, rounds, onfinish, onback } = &props;

    let round_mapper = |round: &DraftRound| {
        html! {
            <Column size={ColumnSize::Is3}>
            <DynImage src={round.image.as_ref().map(ImageData::src).unwrap_or_default()} height=20/>
            <p class="has-text-centered"> <b>{round.answer.clone()}</b> </p>
            </Column>
        }
    };

    html! {
        <Section>
        <Container>

        <Hero color={Color::Primary}>
            <Title> {quiz.title.clone()} </Title>
            <Subtitle> {quiz.description.clone()} </Subtitle>
        </Hero>

        <Box class="mt-5">
        <Columns multiline=true>
            { for rounds.iter().map(round_mapper) }
        </Columns>
        </Box>

        <Buttons>
            <Button color={Color::Danger} light=true onclick={props.onback.clone()}> {"back"} </Button>
            <Button color={Color::Primary} onclick={props.onfinish.clone()}> {"finish"} </Button>
        </Buttons>
        </Container>
        </Section>
    }
}
