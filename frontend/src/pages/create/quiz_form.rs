use crate::graphql::DraftQuiz;
use crate::structs::ImageData;
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
    const NAME_DEFAULT: &str = "Cities";
    const CREATOR_DEFAULT: &str = "John Doe";
    const DESCRIPTION_DEFAULT: &str = "The best quiz";

    let Props { form, editing } = &props;
    let DraftQuiz { name, creator, description, image } = &form.inner;

    let errors = form.errors();
    let filename = image.as_ref().map(ImageData::name);
    let fullwidth = filename.as_ref().is_some();

    let onupload = form.change(|x, f: Vec<web_sys::File>| match f.len() {
        1 => x.image = ImageData::from_local(&f[0]),
        _ => {} // TODO: error
    });

    let left = html! {<Title> {"Overview"} </Title>};
    let right =
        || html! {<Button color={Color::Danger} onclick={form.reset()}> {"delete"} </Button>};

    html! {
        <>
        <Level left={left} right={editing.then(right)} />

        <SimpleField label="Quiz Name" help={errors.get("name").cloned()} help_color={Color::Danger}>
            <Input oninput={form.field(|x| &mut x.name)} value={name.clone()} placeholder={NAME_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Creator" help={errors.get("creator").cloned()} help_color={Color::Danger}>
            <Input oninput={form.field(|x| &mut x.creator)} value={creator.clone()} placeholder={CREATOR_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Description" help={errors.get("description").cloned()} help_color={Color::Danger}>
            <Textarea oninput={form.field(|x| &mut x.description)} value={description.clone()} placeholder={DESCRIPTION_DEFAULT} />
        </SimpleField>

        <SimpleField label="Image" help={errors.get("image").cloned()} help_color={Color::Danger}>
            <File fullwidth={fullwidth} filename={filename} onupload={onupload}/>
        </SimpleField>

        <Buttons>
            <Button color={Color::Primary} light=true onclick={form.cancel()}> {"cancel"} </Button>
            <Button color={Color::Primary} disabled={!errors.is_empty()} onclick={form.submit()}> {"continue"} </Button>
        </Buttons>
        </>
    }
}
