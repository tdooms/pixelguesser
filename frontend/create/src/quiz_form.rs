use cobul::*;
use std::rc::Rc;
use yew::*;

use api::{DraftQuiz, Image, Resolution};
use components::TagsField;
use cropper::Cropper;
use hasura::Data;
use ywt::callback;

const TITLE: &str = "Cities";
const EXPLANATION: &str = "Guess quickly";
const DESCRIPTION: &str = "The best quiz";
const TAGS: &str = "Europe/Geography/Movies";

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub draft: Option<Rc<DraftQuiz>>,
    pub onsubmit: Callback<DraftQuiz>,
    pub onchange: Callback<DraftQuiz>,
    pub onback: Callback<()>,
}

#[function_component(QuizForm)]
pub fn quiz_form(props: &Props) -> Html {
    let Props { draft, onsubmit, onback, onchange } = props.clone();

    let actions = Actions::new().submit(onsubmit).change(onchange.clone());
    let (form, draft) = use_form(&*draft.unwrap_or_default(), actions);

    let cropper = use_state(|| None);
    let name = use_state(|| None);
    let reader = use_state(|| None);

    let onloaded = callback!(cropper, name; move |image: Image| {
        cropper.set(Some(image.src(Resolution::Original)));
        name.set(image.name());
    });
    let onupload = callback!(onloaded, reader; move |files: Vec<web_sys::File>| {
        let fr = Image::from_file(files[0].clone(), onloaded.clone());
        reader.set(Some(fr))
    });
    let ondone = callback!(form, cropper, name; move |base64| {
        let image = Image::from_base64(base64, (*name).clone());
        form.field(|x| &mut x.image).emit(image);

        cropper.set(None);
        name.set(None);
    });
    let ontags = callback!(onchange, draft; move |tags| {
        let new = DraftQuiz { tags: Data{data: tags}, ..draft.clone()};
        onchange.emit(new);
    });
    let oncancel = callback!(cropper, name; move |_| {
        cropper.set(None);
        name.set(None);
    });

    let DraftQuiz { title, explanation, public, description, image, .. } = draft;

    let name = image.name().unwrap_or(format!("{}.jpg", title.to_lowercase()));
    let filename = (!image.is_none()).then(move || name);
    let fullwidth = !image.is_none();

    let modal = match (*cropper).clone() {
        Some(src) => html! {<Cropper {src} {ondone} {oncancel} height=450 width=600/>},
        None => html! {},
    };

    html! {
        <>
        {modal}

        <simple::Field label="Quiz Title" help={form.error("title")}>
            <Input oninput={form.field(|x| &mut x.title)} value={title.clone()} placeholder={TITLE}/>
        </simple::Field>

        <simple::Field label="Description" help={form.error("description")}>
            <Input oninput={form.field(|x| &mut x.description)} value={description.clone()} placeholder={DESCRIPTION} />
        </simple::Field>

        <simple::Field label="Explanation">
            <Input oninput={form.field(|x| &mut x.explanation)} value={explanation.clone()} placeholder={EXPLANATION}/>
        </simple::Field>

        <TagsField onchange={ontags} placeholder={TAGS}/>

        <simple::Field label="Image" help={form.error("image")}>
            <File accept={"image/*"} {fullwidth} {filename} {onupload}/>
        </simple::Field>

        <simple::Field label="Public">
            <Checkbox name="" checked={public} onchange={form.field(|x| &mut x.public)}>
            {" Make this quiz public"}
            </Checkbox>
        </simple::Field>

        <Buttons>
            <Button color={Color::Info} light=true onclick={onback}>
                <span> {"Back"} </span>
            </Button>
            <Button color={Color::Info} disabled={!form.can_submit()} onclick={form.submit()}>
                <span> {"Rounds"} </span>
            </Button>
        </Buttons>
        </>
    }
}
