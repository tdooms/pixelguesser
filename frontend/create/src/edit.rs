use std::rc::Rc;

use cobul::*;
use validator::ValidationErrors;
use yew::*;
use ywt::callback;

use api::{Algorithm, Guesses, Image, Points, Round};
use components::{Sidebar, SidebarAlignment};
use shared::use_form;

fn generate_error_message(errors: &[ValidationErrors]) -> Option<String> {
    let sum: usize = errors.iter().map(|x| x.errors().len()).sum();
    match sum {
        0 => None,
        1 => {
            let idx = errors.iter().position(|x| !x.is_empty()).unwrap();
            let field = errors[idx].errors().keys().next().unwrap();
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

    pub errors: Rc<Vec<ValidationErrors>>,
}

#[function_component(RoundEdit)]
pub fn round_edit(props: &Props) -> Html {
    let Props { round, back, done, input, errors } = props.clone();
    let form = use_form(round.clone(), input.clone());

    let click = callback!(round, input; move |_| {
        input.emit(Rc::new(Round { image: Image::default(), ..(*round).clone() }));
    });

    let has_image = !round.image.is_empty();
    let disabled = !errors.iter().all(|x| x.is_empty());
    let tooltip = generate_error_message(&&errors);

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
            <Slider<u64> class="mb-0" range={50..200} step=5 model={form.speed()} fullwidth=true tooltip=true fmt="{}%" label_width=3.5 />
            <div class="is-flex is-justify-content-space-between">
                <p> {"Slow"} </p> <p> {"Normal"} </p> <p> {"Fast"} </p>
            </div>
        </simple::Field>

        <Block/>
        <simple::Button fullwidth=true {click} light=true color={Color::Danger} hidden={!has_image} text="Remove image" />
        </div>
    };

    let footer = html! {
        <Buttons class="mt-auto pr-5 pl-4 py-2 mr-1">
            <simple::Button fullwidth=true color={Color::Info} click={done} {disabled} {tooltip} style="cursor:pointer" text={"Overview"} />
            <simple::Button fullwidth=true color={Color::Info} click={back} light=true text={"Quiz"} />
        </Buttons>
    };

    html! {
        <Sidebar size={ColumnSize::IsNarrow} alignment={SidebarAlignment::Right} class="p-0" overflow=false {footer}>
            {body}
        </Sidebar>
    }
}
