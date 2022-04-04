use cobul::props::{Color, ColumnSize};
use cobul::*;
use yew::prelude::*;

use api::{DraftQuiz, DraftRound, Resolution};

use crate::state::UseCreateStateHandle;
use crate::{callback, CreateStage, Route};
use yew_router::prelude::use_navigator;

#[function_component(Summary)]
pub fn summary() -> Html {
    let state = use_context::<UseCreateStateHandle>().unwrap();
    let navigator = use_navigator().unwrap();

    let ondone = callback!(navigator; move |_| navigator.push(Route::Overview));
    let onback = callback!(state; move |_| state.set_stage(CreateStage::Rounds));

    let DraftQuiz { title, description, .. } = state.quiz();
    let rounds = state.rounds();

    let round_mapper = |round: &DraftRound| {
        html! {
            <Column size={ColumnSize::Is3}>
            <DynImage src={api::Image::src_or_placeholder(round.image.as_ref(), Resolution::Thumbnail)} height=20/>
            <p class="has-text-centered"> <b>{round.answer.clone()}</b> </p>
            </Column>
        }
    };

    html! {
        <Section>
        <Container>

        <Hero color={Color::Primary}>
            <Title> {title} </Title>
            <Subtitle> {description} </Subtitle>
        </Hero>

        <Box class="mt-5">
        <Columns multiline=true>
            { for rounds.iter().map(round_mapper) }
        </Columns>
        </Box>

        <Buttons>
            <Button color={Color::Info} outlined=true onclick={onback}>
                <Icon icon={Icons::ArrowLeft}/> <span> {"Rounds"} </span>
            </Button>
            <Button color={Color::Primary} onclick={ondone}>
                <span> {"Submit"} </span>
            </Button>
        </Buttons>
        </Container>
        </Section>
    }
}
