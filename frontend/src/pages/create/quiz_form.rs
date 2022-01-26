use crate::graphql::{DraftQuiz, ImageData};
use cobul::props::Color;
use cobul::*;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub form: Form<DraftQuiz>,
    pub editing: bool,
}

#[function_component(QuizForm)]
pub fn quiz_form(props: &Props) -> Html {
    const TITLE_DEFAULT: &str = "Cities";
    const EXPLANATION_DEFAULT: &str = "Guess quick";
    const DESCRIPTION_DEFAULT: &str = "The best quiz";

    let Props { form, editing } = &props;
    let DraftQuiz { title, explanation, public, description, image, .. } = &form.inner;

    let errors = form.errors();
    let filename = image.as_ref().map(ImageData::name);
    let fullwidth = filename.as_ref().is_some();

    let onupload = form.onchange(|x, f: Vec<web_sys::File>| match f.len() {
        1 => x.image = ImageData::from_local(&f[0]),
        _ => {} // TODO: error
    });

    let left = html! {<Title> {"Overview"} </Title>};
    let right =
        || html! {<Button color={Color::Danger} onclick={form.onreset()}> {"delete"} </Button>};

    html! {
        <>
        <Level left={left} right={editing.then(right)} />

        <SimpleField label="Quiz Title" help={errors.get("title").cloned()} help_color={Color::Danger}>
            <Input oninput={form.onfield(|x| &mut x.title)} value={title.clone()} placeholder={TITLE_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Description" help={errors.get("description").cloned()} help_color={Color::Danger}>
            <Textarea oninput={form.onfield(|x| &mut x.description)} value={description.clone()} placeholder={DESCRIPTION_DEFAULT} />
        </SimpleField>

        <SimpleField label="Explanation" help={errors.get("explanation").cloned()} help_color={Color::Danger}>
            <Input oninput={form.onfield(|x| &mut x.explanation)} value={explanation.clone()} placeholder={EXPLANATION_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Image" help={errors.get("image").cloned()} help_color={Color::Danger}>
            <File fullwidth={fullwidth} filename={filename} onupload={onupload}/>
        </SimpleField>

        <Buttons>
            <Button color={Color::Danger} light=true onclick={form.oncancel()}> {"back"} </Button>
            <Button color={Color::Primary} disabled={!errors.is_empty()} onclick={form.onsubmit()}> {"next"} </Button>
        </Buttons>
        </>
    }
}
