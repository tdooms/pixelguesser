use cobul::*;
use components::{DynImage, Fit, Height};
use std::rc::Rc;
use yew::*;

use api::{DraftQuiz, DraftRound, Resolution};

use crate::state::Action;
use crate::Stage;
use shared::use_form;
use ywt::callback;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub change: Callback<Stage>,
    pub action: Callback<Action>,
    pub draft: Rc<DraftQuiz>,
}

pub fn view_round(round: &DraftRound) -> Html {
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
    let Props { onstage, onaction, draft } = props.clone();
    let DraftQuiz { public, .. } = *draft;

    let onchange = props.onaction.reform(Action::Quiz);
    let form = use_form(draft.clone(), onchange);

    let ondone = callback!(onstage, onaction; move |_| {
        onaction.emit(Action::Submit);
        onstage.emit(Stage::Done)
    });
    let onback = callback!(onstage; move |_| onstage.emit(Stage::Rounds));

    html! {
        <Section>
        <Container>

        <Hero color={Color::Primary}>
            <Title> {&draft.title} </Title>
            <Subtitle> {&draft.description} </Subtitle>
        </Hero>

        <Box class="mt-5">
            <simple::Field label="Public">
                <Checkbox id="1" label="public" onchange={form.change(|x| &mut x.public)} checked={public}/>
            </simple::Field>
        </Box>

        <Box class="mt-5">
        <Columns multiline=true>
            { for draft.rounds.data.iter().map(view_round) }
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
