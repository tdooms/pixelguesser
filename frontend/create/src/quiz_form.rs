use agents::{AuthAgent, ErrorAgent};
use api::DraftQuiz;
use cobul::props::Color;
use cobul::*;
use shared::Error;
use yew::prelude::*;
use yew_agent::use_bridge;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub form: Form<DraftQuiz>,
    pub editing: bool,
}

#[function_component(QuizForm)]
pub fn quiz_form(props: &Props) -> Html {
    const TITLE_DEFAULT: &str = "Cities";
    const EXPLANATION_DEFAULT: &str = "Guess quickly";
    const DESCRIPTION_DEFAULT: &str = "The best quiz";

    let Props { form, editing } = &props;
    let DraftQuiz { title, explanation, public, description, image, .. } = &form.inner;

    let bridge = use_bridge::<ErrorAgent, _>(|_| ());

    let errors = form.errors();
    let filename = image.as_ref().map(api::Image::name);
    let fullwidth = filename.as_ref().is_some();

    let onupload = form.onchange(move |x, f: Vec<web_sys::File>| match f.len() {
        1 => x.image = api::Image::from_local(&f[0]),
        _ => bridge.send(Error::MultipleFiles),
    });

    let left = html! {<Title> {"Overview"} </Title>};
    let right = || html! {<Button color={Color::Danger} onclick={form.onreset()}> {"Delete Quiz"} </Button>};

    let explanation_help = "Players will see this when they start the quiz.";

    html! {
        <>
        <Level left={left} right={editing.then(right)} />

        <SimpleField label="Quiz Title" help={errors.get("title").cloned()} help_color={Color::Danger}>
            <Input oninput={form.onfield(|x| &mut x.title)} value={title.clone()} placeholder={TITLE_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Description" help={errors.get("description").cloned()} help_color={Color::Danger}>
            <Input oninput={form.onfield(|x| &mut x.description)} value={description.clone()} placeholder={DESCRIPTION_DEFAULT} />
        </SimpleField>

        <SimpleField label="Explanation" help={explanation_help}>
            <Input oninput={form.onfield(|x| &mut x.explanation)} value={explanation.clone()} placeholder={EXPLANATION_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Image" help={errors.get("image").cloned()} help_color={Color::Danger}>
            <File fullwidth={fullwidth} filename={filename} onupload={onupload}/>
        </SimpleField>

        <Buttons>

            <Button color={Color::Info} outlined=true onclick={form.oncancel()}>
                <Icon icon={Icons::ArrowLeft}/> <span> {"Back"} </span>
            </Button>
            <Button color={Color::Primary} light=true disabled={!errors.is_empty()} onclick={form.onsubmit()}>
                <Icon icon={Icons::ArrowRight}/> <span> {"Edit Rounds"} </span>
            </Button>
        </Buttons>
        </>
    }
}
