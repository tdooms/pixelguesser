use std::rc::Rc;

use cobul::*;
use cropper::Cropper;
use shared::callback;
use yew::*;

use api::{Image, Quiz};
use components::{QuizCard, TagsField};
use shared::{use_auth, use_form, use_toast, Forbidden};

use crate::picker::Picker;
use crate::state::Action;
use crate::Stage;

const TITLE: &str = "Cities";
const EXPLANATION: &str = "Guess quickly";
const DESCRIPTION: &str = "The best quiz";
const TAGS: &str = "Europe/Geography/Movies";

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub change: Callback<Stage>,
    pub action: Callback<Action>,
    pub quiz: Rc<Quiz>,
}

#[function_component(QuizPage)]
pub fn quiz_page(props: &Props) -> Html {
    let Props { change, action, quiz } = props.clone();

    let user = use_auth().user();
    let toast = use_toast();

    let creator = match toast.maybe(user.ok_or(Forbidden("logged in"))) {
        Some(user) => user.nickname.clone(),
        None => return html! {},
    };

    // let cropper: UseStateHandle<Option<Rc<String>>> = use_state(|| None);
    // let name = use_state(|| None);
    let active = use_state(|| false);

    let form = use_form(quiz.clone(), action.reform(Action::Quiz));

    let delete = callback!(action, change; move |_| {
        action.emit(Action::Delete);
        change.emit(Stage::Back);
    });
    let back = callback!(change; move |_| {
        change.emit(Stage::Back)
    });
    // let upload = callback!(name, cropper; move |files: Vec<web_sys::File>| {
    //     ywt::spawn!(name, cropper; async move {
    //         let image = Image::from_local(files[0].clone()).await.unwrap();
    //         cropper.set(Some(image.src(Resolution::Original)));
    //         name.set(image.name());
    //     })
    // });
    // let done = callback!(form, cropper, name; move |base64| {
    //     let image = Image::from_base64(base64, (*name).clone());
    //     form.change(|x| &mut x.image).emit(image);
    //
    //     cropper.set(None);
    //     name.set(None);
    // });
    // let cancel = callback!(cropper, name; move |_| {
    //     cropper.set(None);
    //     name.set(None);
    // });

    // let activate = callback!(active; move |_| active.set(!*active));

    // let name = image.name().unwrap_or(format!("{}.jpg", title.to_lowercase()));
    // let filename = (!image.is_empty()).then(move || name);
    // let fullwidth = !image.is_empty();

    // let modal = match (*cropper).clone() {
    //     Some(src) => html! {<Cropper {src} {done} {cancel} height=450 width=600/>},
    //     None => html! {},
    // };

    let left = html! {<Title> {"Overview"} </Title>};
    let right = quiz
        .quiz_id
        .map(|_| html! {<simple::Button color={Color::Danger} click={delete} text="Delete" /> })
        .unwrap_or_default();

    let body = html! {
        <>
        <Modal active={*active} >
            <Picker change={Callback::noop()} narrow=true />
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

    html! {
        <Section>
        <Container>
            // {modal}
            <Columns>
                <Column> {body} </Column>
                <Column size={ColumnSize::Is1} />
                <Column size={ColumnSize::Is4}> <QuizCard {quiz} {creator}/> </Column>
            </Columns>
            <Buttons>
                <simple::Button color={Color::Info} light=true click={back} text="Back" />
                <simple::Button color={Color::Info} disabled={!form.errors().is_empty()} click={change.reform(|_| Stage::Rounds)} text="Rounds" />
            </Buttons>
        </Container>
        </Section>
    }
}
