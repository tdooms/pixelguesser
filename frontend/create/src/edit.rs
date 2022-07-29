use cobul::*;
use components::{Sidebar, SidebarAlignment};
use std::rc::Rc;
use validator::ValidationErrors;
use yew::*;
use ywt::callback;

use api::{Algorithm, DraftRound, Guesses, Image, Points};

fn generate_error_message(errors: &[Option<ValidationErrors>]) -> Option<String> {
    let sum: usize = errors.iter().filter_map(|x| x.as_ref().map(|y| y.errors().len())).sum();
    match sum {
        0 => None,
        1 => {
            let idx = errors.iter().position(|x| x.is_some()).unwrap();
            let field = errors[idx].as_ref().unwrap().errors().keys().next().unwrap();
            Some(format!("Missing {field} on round {idx}"))
        }
        _ => Some("Multiple missing images or answers".to_owned()),
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub round: Rc<DraftRound>,
    pub errors: Rc<Vec<Option<ValidationErrors>>>,

    pub onback: Callback<()>,
    pub ondone: Callback<()>,
    pub onedit: Callback<Rc<DraftRound>>,
}

#[function_component(RoundEdit)]
pub fn round_edit(props: &Props) -> Html {
    let Props { round, onback, ondone, onedit, errors } = props.clone();

    let onclick = callback!(round, onedit; move |_| {
        onedit.emit(Rc::new(DraftRound { image: Image::default(), ..(*round).clone() }));
    });

    let actions = Actions::new().change(onedit.clone());
    let form = use_form(round.clone(), actions);

    let color = {
        let form = form.clone();
        move |name: &str| form.error(name).map(|_| Color::Danger)
    };

    let DraftRound { answer, points, guesses, speed, algorithm, .. } = (*round).clone();
    let has_image = !props.round.image.is_empty();
    let disabled = errors.iter().any(|x| x.is_some());
    let tooltip = generate_error_message(&&errors);

    let footer = html! {
        <Buttons class="mt-auto pr-5 pl-4 py-2">
            <Button fullwidth=true color={Color::Info} onclick={ondone} {disabled} {tooltip} style="cursor:pointer" class="mr-1">
                <span> {"Overview"} </span>
            </Button>
            <Button fullwidth=true color={Color::Info} light=true onclick={onback} class="mr-1">
                <span> {"Quiz"} </span>
            </Button>
        </Buttons>
    };

    let form_body = html! {
        <div class="pt-5 pl-4 pr-5">
        <simple::Field label="Answer" help={form.error("answer")} >
            <Input oninput={form.field(|x| &mut x.answer)} value={answer} color={color("answer")}/>
        </simple::Field>

        <simple::Field label="Points" help={form.error("points")} >
            <simple::Tabs<Points> fullwidth=true toggle=true onclick={form.field(|x| &mut x.points)} value={points}/>
        </simple::Field>

        <simple::Field label="Guesses" help={form.error("guesses")}>
            <simple::Tabs<Guesses> fullwidth=true toggle=true onclick={form.field(|x| &mut x.guesses)} value={guesses}/>
        </simple::Field>

        <simple::Field label="Algorithm">
            <simple::Dropdown<Algorithm> onchange={form.field(|x| &mut x.algorithm)} value={algorithm} fullwidth=true/>
        </simple::Field>

        <simple::Field label="Speed">
            <Slider<u64> class="mb-0" range={50..200} id="456" step=5 value={speed} oninput={form.field(|x| &mut x.speed)} fullwidth=true tooltip=true fmt="{}%" labelwidth=3.5/>
            <div class="is-flex is-justify-content-space-between">
                <p> {"Slow"} </p>
                <p> {"Normal"} </p>
                <p> {"Fast"} </p>
            </div>
        </simple::Field>

        <Block/>
        <Button fullwidth=true {onclick} light=true color={Color::Danger} hidden={!has_image}>
        {"Remove image"}
        </Button>

        </div>
    };

    html! {
        <Sidebar size={ColumnSize::IsNarrow} alignment={SidebarAlignment::Right} class="p-0" overflow=false {footer}>
            {form_body}
        </Sidebar>
    }
}
