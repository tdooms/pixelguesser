use crate::picker::Picker;
use cobul::*;
use std::rc::Rc;
use yew::*;

use api::{Image, Quiz, Resolution};
use components::{QuizCard, TagsField};
use cropper::Cropper;
use shared::{use_auth, use_form, use_toast, Forbidden};
use ywt::callback;

use crate::state::Action;
use crate::Stage;

const TITLE: &str = "Cities";
const EXPLANATION: &str = "Guess quickly";
const DESCRIPTION: &str = "The best quiz";
const TAGS: &str = "Europe/Geography/Movies";

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub stage: Callback<Stage>,
    pub action: Callback<Action>,
    pub quiz: Rc<Quiz>,
}

#[function_component(QuizPage)]
pub fn quiz_page(props: &Props) -> Html {
    let Props { stage, action, quiz } = props.clone();

    let user = use_auth().user();
    let toast = use_toast();

    let creator = match toast.maybe(user.ok_or(Forbidden)) {
        Some(user) => user.nickname.clone(),
        None => return html! {},
    };

    let cropper = use_state(|| None);
    let name = use_state(|| None);
    let active = use_state(|| false);

    let form = use_form(draft.clone(), onaction.reform(Action::Quiz));

    let ondelete = callback!(onaction, onstage; move |_| {
        onaction.emit(Action::Delete);
        onstage.emit(Stage::Back);
    });
    let onback = callback!(onstage; move |_| {
        onstage.emit(Stage::Back)
    });
    let onupload = callback!(name, cropper; move |files: Vec<web_sys::File>| {
        ywt::spawn!(name, cropper; async move {
            let image = Image::from_local(files[0].clone()).await.unwrap();
            cropper.set(Some(image.src(Resolution::Original)));
            name.set(image.name());
        })
    });
    let ondone = callback!(form, cropper, name; move |base64| {
        let image = Image::from_base64(base64, (*name).clone());
        form.change(|x| &mut x.image).emit(image);

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

    let onactive = callback!(active; move |_| active.set(!*active));

    let Quiz { id, title, explanation, description, image, .. } = (*draft).clone();
    let name = image.name().unwrap_or(format!("{}.jpg", title.to_lowercase()));
    let filename = (!image.is_empty()).then(move || name);
    let fullwidth = !image.is_empty();

    let modal = match (*cropper).clone() {
        Some(src) => html! {<Cropper {src} {ondone} {oncancel} height=450 width=600/>},
        None => html! {},
    };

    let left = html! {<Title> {"Overview"} </Title>};
    let right = id
        .map(|_| html! {<simple::Button color={Color::Danger} click={ondelete} text="Delete" /> })
        .unwrap_or_default();

    let body = html! {
        <>
        <Modal active={*active} >
            <Picker onchange={Callback::noop()} narrow=true />
        </Modal>

        <Level {left} {right} />

        <simple::Field label="Quiz Title" help={form.error("title")}>
            <Input model={form.title()} placeholder={TITLE}/>
        </simple::Field>

        <simple::Field label="Description" help={form.error("description")}>
            <Input model={form.description()} placeholder={DESCRIPTION} />
        </simple::Field>

        <simple::Field label="Explanation">
            <Input model={form.explanation()} placeholder={EXPLANATION}/>
        </simple::Field>

        <TagsField model={form.tags()} placeholder={TAGS}/>

        // <simple::Field label="Image" help={form.error("image")}>
        //     <File accept={"image/*"} {fullwidth} {filename} {onupload}/>
        // </simple::Field>
        // <Button click={onactive}> {"Image"} </Button>
        </>
    };

    let buttons = html! {
        <Buttons>
            <Button color={Color::Info} light=true onclick={onback}>
            <span> {"Back"} </span>
            </Button>
            <Button color={Color::Info} disabled={!form.errors().is_empty()} onclick={onstage.reform(|_| Stage::Rounds)}>
            <span> {"Rounds"} </span>
            </Button>
        </Buttons>
    };

    html! {
        <Section>
        <Container>
            {modal}
            <Columns>
                <Column> {body} </Column>
                <Column size={ColumnSize::Is1} />
                <Column size={ColumnSize::Is4}> <QuizCard {quiz} {creator}/> </Column>
            </Columns>
            {buttons}
        </Container>
        </Section>
    }
}
