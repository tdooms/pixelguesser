use cobul::props::Color;
use cobul::*;
use futures::FutureExt;
use yew::prelude::*;

use api::DraftQuiz;
use shared::{async_callback, Error, Errors};

const TITLE_DEFAULT: &str = "Cities";
const EXPLANATION_DEFAULT: &str = "Guess quickly";
const DESCRIPTION_DEFAULT: &str = "The best quiz";
const EXPLANATION_HELP: &str = "Players will see this when they start the quiz.";

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub form: Form<DraftQuiz>,
    pub editing: bool,
}

#[function_component(QuizForm)]
pub fn quiz_form(props: &Props) -> Html {
    let Props { form, editing } = &props;
    let DraftQuiz { title, explanation, description, image, .. } = &form.inner;

    let error_map = form.errors();
    let filename = image.as_ref().map(api::Image::name);
    let fullwidth = filename.as_ref().is_some();

    let errors = use_context::<Errors>().unwrap();

    let onuploaded = form.onfield(|x| &mut x.image);

    let onupload = Callback::from(move |files: Vec<web_sys::File>| {
        if let 1 = files.len() {
            let fut = api::Image::from_local(files[0].clone());
            async_callback(fut.map(Option::Some), onuploaded.clone())
        } else {
            errors.emit(Error::MultipleFiles)
        }
    });

    let left = html! {<Title> {"Overview"} </Title>};
    let right = || html! {<Button color={Color::Danger} onclick={form.onreset()}> {"Delete Quiz"} </Button>};

    html! {
        <>
        <Level {left} right={editing.then(right)} />

        <SimpleField label="Quiz Title" help={error_map.get("title").cloned()} help_color={Color::Danger}>
            <Input oninput={form.onfield(|x| &mut x.title)} value={title.clone()} placeholder={TITLE_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Description" help={error_map.get("description").cloned()} help_color={Color::Danger}>
            <Input oninput={form.onfield(|x| &mut x.description)} value={description.clone()} placeholder={DESCRIPTION_DEFAULT} />
        </SimpleField>

        <SimpleField label="Explanation" help={EXPLANATION_HELP}>
            <Input oninput={form.onfield(|x| &mut x.explanation)} value={explanation.clone()} placeholder={EXPLANATION_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Image" help={error_map.get("image").cloned()} help_color={Color::Danger}>
            <File {fullwidth} {filename} {onupload}/>
        </SimpleField>

        <Buttons>
            <Button color={Color::Info} outlined=true onclick={form.oncancel()}>
                <Icon icon={Icons::ArrowLeft}/> <span> {"Back"} </span>
            </Button>
            <Button color={Color::Primary} light=true disabled={!error_map.is_empty()} onclick={form.onsubmit()}>
                <Icon icon={Icons::ArrowRight}/> <span> {"Edit Rounds"} </span>
            </Button>
        </Buttons>
        </>
    }
}
