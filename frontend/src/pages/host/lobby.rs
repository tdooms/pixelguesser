use cobul::props::{Color, ColumnSize, HeroSize};
use cobul::*;
use sessions::Session;
use yew::prelude::*;

use crate::graphql::Quiz;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub session: Session,
    pub code: String,
    pub quiz: Quiz,
}

#[function_component(Lobby)]
pub fn lobby(props: &Props) -> Html {
    let Props { session, code, quiz } = &props;

    let players = session.players.iter().map(|player| {
        html! { <Column size={ColumnSize::IsNarrow}> <Box> {&player.name} </Box> </Column> }
    });

    let subtitle = match session.has_manager {
        true => "quiz master present",
        false => "no quiz master",
    };

    html! {
        <>
            <Hero>
                <Title> {quiz.name.clone()} </Title>
                <Subtitle> {subtitle} </Subtitle>
            </Hero>


            <Hero color={Color::Primary} size={HeroSize::Medium}>
                <Container extra="has-text-centered">
                    <Title> {code} </Title>
                </Container>
            </Hero>

            <Columns multiline=true centered=true extra="mt-5">
                { for players }
            </Columns>
        </>
    }
}
