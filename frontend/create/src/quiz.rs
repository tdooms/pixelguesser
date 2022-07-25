use cobul::*;
use std::rc::Rc;
use yew::*;

use api::{DraftQuiz, Image, Resolution};
use components::{QuizCard, TagsField, View};
use cropper::Cropper;
use hasura::Data;
use shared::Auth;
use ywt::callback;

use crate::state::Action;
use crate::Stage;

const TITLE: &str = "Cities";
const EXPLANATION: &str = "Guess quickly";
const DESCRIPTION: &str = "The best quiz";
const TAGS: &str = "Europe/Geography/Movies";

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onstage: Callback<Stage>,
    pub onaction: Callback<Action>,

    pub draft: Rc<DraftQuiz>,
    pub has_delete: bool,
}

#[function_component(QuizPage)]
pub fn quiz_page(props: &Props) -> Html {
    let Props { onstage, onaction, draft, has_delete } = props.clone();

    let creator = match use_context::<Auth>().unwrap().user() {
        Ok(user) => user.nickname,
        Err(_) => return html! { "not allowed" },
    };

    let cropper = use_state(|| None);
    let name = use_state(|| None);
    let reader = use_state(|| None);

    let actions = Actions::new()
        .submit(onstage.reform(|_| Stage::Rounds))
        .change(onaction.reform(Action::Quiz));

    let form = use_form(draft.clone(), actions);

    let ondelete = callback!(onaction, onstage; move |_| {
        onaction.emit(Action::Delete);
        onstage.emit(Stage::Back);
    });
    let onback = callback!(onstage; move |_| {
        onstage.emit(Stage::Back)
    });
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
    let ontags = callback!(onaction, draft; move |tags| {
        let new = DraftQuiz { tags: Data{data: tags}, ..(*draft).clone()};
        onaction.emit(Action::Quiz(Rc::new(new)));
    });
    let oncancel = callback!(cropper, name; move |_| {
        cropper.set(None);
        name.set(None);
    });

    let DraftQuiz { title, explanation, description, image, .. } = (*draft).clone();
    let name = image.name().unwrap_or(format!("{}.jpg", title.to_lowercase()));
    let filename = (!image.is_none()).then(move || name);
    let fullwidth = !image.is_none();

    let modal = match (*cropper).clone() {
        Some(src) => html! {<Cropper {src} {ondone} {oncancel} height=450 width=600/>},
        None => html! {},
    };

    let delete = || html! {<Button color={Color::Danger} onclick={ondelete}> {"Delete"} </Button>};
    let left = html! {<Title> {"Overview"} </Title>};
    let right = has_delete.then(|| delete()).unwrap_or_default();

    let form_body = html! {
        <>
        <Level {left} {right} />

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
        </>
    };

    let buttons = html! {
        <Buttons>
            <Button color={Color::Info} light=true onclick={onback}>
            <span> {"Back"} </span>
            </Button>
            <Button color={Color::Info} disabled={!form.can_submit()} onclick={form.submit()}>
            <span> {"Rounds"} </span>
            </Button>
        </Buttons>
    };

    html! {
        <Section>
        <Container>
            {modal}
            <Columns>
                <Column> {form_body} </Column>
                <Column size={ColumnSize::Is1} />
                <Column size={ColumnSize::Is4}> <QuizCard view={View::Preview{draft, creator}}/> </Column>
            </Columns>
            {buttons}
        </Container>
        </Section>
    }
}
