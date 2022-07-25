use cobul::*;
use components::{DynImage, Fit, Height};
use std::rc::Rc;
use yew::*;

use api::{DraftQuiz, DraftRound, Resolution};

use crate::state::Action;
use crate::Stage;
use ywt::callback;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onstage: Callback<Stage>,
    pub onaction: Callback<Action>,
    pub draft: Rc<DraftQuiz>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct RoundProps {
    pub round: Rc<DraftRound>,
}

#[function_component(SummaryRound)]
pub fn summary_round(RoundProps { round }: &RoundProps) -> Html {
    let style = "border-width:thin;border-style:solid;border-radius:5px;border-color:lightgray";
    html! {
        <Column size={ColumnSize::Is3}>
        <DynImage {style} src={round.image.src(Resolution::Card)} height={Height::Vh(20)} fit={Fit::Cover} border=true/>
        <p class="has-text-centered"> <b>{round.answer.clone()}</b> </p>
        </Column>
    }
}

#[function_component(Summary)]
pub fn summary(props: &Props) -> Html {
    let Props { onstage, onaction, draft } = props.clone();
    let DraftQuiz { public, .. } = *draft;

    let actions = Actions::new();
    let form = use_form(draft.clone(), actions);

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
                <Checkbox id="1" label="public" onchange={form.field(|x| &mut x.public)} checked={public}/>
            </simple::Field>
        </Box>

        <Box class="mt-5">
        <Columns multiline=true>
            { for draft.rounds.data.iter().map(|x| html! {<SummaryRound round={Rc::new(x.clone())} />}) }
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
