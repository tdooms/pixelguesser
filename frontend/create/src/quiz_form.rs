use cobul::props::Color;
use cobul::*;
use yew::prelude::*;

use api::DraftQuiz;

const TITLE_DEFAULT: &str = "Cities";
const EXPLANATION_DEFAULT: &str = "Guess quickly";
const DESCRIPTION_DEFAULT: &str = "The best quiz";
// const EXPLANATION_HELP: &str = "Players will see this when they start the quiz.";

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub quiz: Option<DraftQuiz>,
    pub onsubmit: Callback<DraftQuiz>,
    pub onchange: Callback<DraftQuiz>,
    pub onback: Callback<()>,
}

async fn make_image(mut quiz: DraftQuiz, files: Vec<web_sys::File>) -> DraftQuiz {
    quiz.image = api::Image::from_file(files[0].clone()).await;
    quiz
}

#[function_component(QuizForm)]
pub fn quiz_form(props: &Props) -> Html {
    log::trace!("quiz form render");
    let Props { quiz, onsubmit, onback, onchange } = props.clone();

    let actions = Actions::new().submit(onsubmit).change(onchange);
    let (form, quiz) = use_form(&quiz.unwrap_or_default(), actions);
    let DraftQuiz { title, explanation, public, description, image, .. } = quiz;

    let filename = image.name().unwrap_or(format!("{}.jpg", title.to_lowercase()));
    let fullwidth = !image.is_none();

    html! {
        <>
        <SimpleField label="Quiz Title" help={form.error("title")}>
            <Input oninput={form.field(|x| &mut x.title)} value={title.clone()} placeholder={TITLE_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Description" help={form.error("description")}>
            <Input oninput={form.field(|x| &mut x.description)} value={description.clone()} placeholder={DESCRIPTION_DEFAULT} />
        </SimpleField>

        <SimpleField label="Explanation">
            <Input oninput={form.field(|x| &mut x.explanation)} value={explanation.clone()} placeholder={EXPLANATION_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Image" help={form.error("image")}>
            <File accept={"image/*"} {fullwidth} {filename} onupload={form.async_field(make_image)}/>
        </SimpleField>

        <SimpleField label="Public">
            <Checkbox name="" checked={public} onchange={form.field(|x| &mut x.public)}>
            {" Make this quiz public"}
            </Checkbox>
        </SimpleField>

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
