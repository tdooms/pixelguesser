use cobul::*;
use std::rc::Rc;
use yew::*;

use api::{DraftQuiz, Image, Resolution};
use cropper::Cropper;
use ywt::callback;

const TITLE_DEFAULT: &str = "Cities";
const EXPLANATION_DEFAULT: &str = "Guess quickly";
const DESCRIPTION_DEFAULT: &str = "The best quiz";
// const EXPLANATION_HELP: &str = "Players will see this when they start the quiz.";

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

    let actions = Actions::new().submit(onsubmit).change(onchange);
    let (form, draft) = use_form(&*draft.unwrap_or_default(), actions);
    let DraftQuiz { title, explanation, public, description, image, .. } = draft;

    let cropper = use_state(|| None);
    let name = use_state(|| None);

    let onloaded = callback!(cropper, name; move |image: Image| {
        cropper.set(Some(image.src(Resolution::Original)));
        name.set(image.name());
    });
    let onupload = callback!(onloaded; move |files: Vec<web_sys::File>| {
        Image::from_file(files[0].clone(), onloaded.clone());
    });

    let ondone = callback!(form, cropper, name; move |base64| {
        let image = Image::from_base64(base64, (*name).clone());
        form.field(|x| &mut x.image).emit(image);

        cropper.set(None);
        name.set(None);
    });

    let oncancel = callback!(cropper, name; move |_| {
        cropper.set(None);
        name.set(None);
    });

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
            <Input oninput={form.field(|x| &mut x.title)} value={title.clone()} placeholder={TITLE_DEFAULT}/>
        </simple::Field>

        <simple::Field label="Description" help={form.error("description")}>
            <Input oninput={form.field(|x| &mut x.description)} value={description.clone()} placeholder={DESCRIPTION_DEFAULT} />
        </simple::Field>

        <simple::Field label="Explanation">
            <Input oninput={form.field(|x| &mut x.explanation)} value={explanation.clone()} placeholder={EXPLANATION_DEFAULT}/>
        </simple::Field>

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
