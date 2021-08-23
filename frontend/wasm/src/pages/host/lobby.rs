use pbs::prelude::*;
use pbs::properties::{Color, ColumnSize, HeroSize};
use yew::prelude::*;

use crate::graphql::Quiz;
use shared::Session;
use yew::utils::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub session: Session,
    pub code: String,
    pub quiz: Quiz,
}

pub struct Lobby {
    props: Props,
}

impl Component for Lobby {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let Props { session, code, quiz } = &self.props;

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
}
