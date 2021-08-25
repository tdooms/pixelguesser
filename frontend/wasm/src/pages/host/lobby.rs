use yew::prelude::*;

use pbs::prelude::*;
use pbs::properties::{Color, ColumnSize, HeroSize};
use shared::Session;

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

    let body = html! {
        <Container extra="has-text-centered">
            <Title> {code} </Title>
        </Container>
    };

    html! {
        <>
            <cbs::TitleHero title={quiz.name.clone()} subtitle={subtitle} />
            <Hero color={Color::Primary} size={HeroSize::Medium} body={body} />

            <Columns multiline=true centered=true extra="mt-5">
                { for players }
            </Columns>
        </>
    }
}
