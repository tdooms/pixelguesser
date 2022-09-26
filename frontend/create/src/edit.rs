use cobul::*;
use components::{Sidebar, SidebarAlignment};
use std::rc::Rc;
use validator::ValidationErrors;
use yew::*;
use ywt::callback;

use api::{Algorithm, Guesses, Image, Points, Round};
use shared::use_form;

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
    pub round: Rc<Round>,
    pub input: Callback<Rc<Round>>,

    pub back: Callback<()>,
    pub done: Callback<()>,

    pub errors: Rc<Vec<Option<ValidationErrors>>>,
}

#[function_component(RoundEdit)]
pub fn round_edit(props: &Props) -> Html {
    let Props { round, back, done, input, errors } = props.clone();

    let click = callback!(round, edit; move |_| {
        edit.emit(Rc::new(Round { image: Image::default(), ..(*round).clone() }));
    });

    let form = use_form(round.clone(), onedit);

    let DraftRound { answer, points, guesses, speed, algorithm, .. } = (*round).clone();
    let has_image = !props.round.image.is_empty();
    let disabled = errors.iter().any(|x| x.is_some());
    let tooltip = generate_error_message(&&errors);

    let footer = html! {
        <Buttons class="mt-auto pr-5 pl-4 py-2 mr-1">
            <simple::Button fullwidth=true color={Color::Info} onclick={ondone} {disabled} {tooltip} style="cursor:pointer" label={"Overview"} />
            <simple::Button fullwidth=true color={Color::Info} light=true onclick={onback} label={"Quiz"} />
        </Buttons>
    };

    let body = html! {
        <div class="pt-5 pl-4 pr-5">
        <simple::Field label="Answer" help={form.error("answer")} >
            <Input model={form.answer()} />
        </simple::Field>

        <simple::Field label="Points" help={form.error("points")} >
            <simple::Tabs<Points> model={form.points()} fullwidth=true toggle=true />
        </simple::Field>

        <simple::Field label="Guesses" help={form.error("guesses")}>
            <simple::Tabs<Guesses> model={form.guesses()} fullwidth=true toggle=true />
        </simple::Field>

        <simple::Field label="Algorithm">
            <simple::Dropdown<Algorithm> model={form.algorithm()} fullwidth=true />
        </simple::Field>

        <simple::Field label="Speed">
            <Slider<u64> class="mb-0" range={50..200} step=5 model={form.speed()} fullwidth=true tooltip=true fmt="{}%" labelwidth=3.5/>
            <div class="is-flex is-justify-content-space-between">
                <p> {"Slow"} </p>
                <p> {"Normal"} </p>
                <p> {"Fast"} </p>
            </div>
        </simple::Field>

        <Block/>
        <simple::Button fullwidth=true {click} light=true color={Color::Danger} hidden={!has_image} label="Remove image" />
        </div>
    };

    html! {
        <Sidebar size={ColumnSize::IsNarrow} alignment={SidebarAlignment::Right} class="p-0" overflow=false {footer}>
            {body}
        </Sidebar>
    }
}
