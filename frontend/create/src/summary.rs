use std::rc::Rc;

use cobul::*;
use yew::*;
use ywt::callback;

use api::{Quiz, Resolution, Round};
use components::{DynImage, Fit, Height};
use shared::use_form;

use crate::state::Action;
use crate::Stage;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub change: Callback<Stage>,
    pub action: Callback<Action>,
    pub quiz: Rc<Quiz>,
}

pub fn view_round(round: &Round) -> Html {
    let style = "border-width:thin;border-style:solid;border-radius:5px;border-color:lightgray";
    html! {
        <Column size={ColumnSize::Is3}>
        <DynImage {style} src={round.image.src(Resolution::Small)} height={Height::Vh(20)} fit={Fit::Cover} border=true/>
        <p class="has-text-centered"> <b>{round.answer.clone()}</b> </p>
        </Column>
    }
}

#[function_component(Summary)]
pub fn summary(props: &Props) -> Html {
    let Props { change, action, quiz } = props.clone();
    let form = use_form(quiz.clone(), action.reform(Action::Quiz));

    let done = callback!(change, action; move |_| {
        action.emit(Action::Submit);
        change.emit(Stage::Done)
    });
    let back = callback!(change; move |_| change.emit(Stage::Rounds));

    html! {
        <Section>
        <Container>

        <Hero color={Color::Info}>
            <Title> {&quiz.title} </Title>
            <Subtitle> {&quiz.description} </Subtitle>
        </Hero>

        <Box class="mt-5">
            <simple::Field label="Public">
                <Checkbox label="public" model={form.public()} />
            </simple::Field>
        </Box>

        <Box class="mt-5">
        <Columns multiline=true>
            { for quiz.rounds.iter().map(view_round) }
        </Columns>
        </Box>

        <Buttons>
            <simple::Button color={Color::Info} click={back} text="Rounds" light=true />
            <simple::Button color={Color::Info} click={done} text="Submit" />
        </Buttons>

        </Container>
        </Section>
    }
}
