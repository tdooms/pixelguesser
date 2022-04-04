use cobul::props::Color;
use cobul::*;
use yew::prelude::*;

use api::DraftQuiz;

const TITLE_DEFAULT: &str = "Cities";
const EXPLANATION_DEFAULT: &str = "Guess quickly";
const DESCRIPTION_DEFAULT: &str = "The best quiz";
const EXPLANATION_HELP: &str = "Players will see this when they start the quiz.";

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub quiz: Option<DraftQuiz>,
    pub onsubmit: Callback<DraftQuiz>,
    pub onchange: Callback<DraftQuiz>,
    pub onback: Callback<()>,
}

async fn make_image(mut quiz: DraftQuiz, files: Vec<web_sys::File>) -> DraftQuiz {
    quiz.image = Some(api::Image::from_local(files[0].clone()).await);
    quiz
}

#[function_component(QuizForm)]
pub fn quiz_form(props: &Props) -> Html {
    log::trace!("quiz form render");
    let Props { quiz, onsubmit, onback, onchange } = props.clone();

    let actions = Actions::new().submit(onsubmit).change(onchange);
    let (form, quiz) = use_form(&quiz.unwrap_or_default(), actions);
    let DraftQuiz { title, explanation, public: _, description, image, .. } = quiz;

    let filename = image.as_ref().map(api::Image::name);
    let fullwidth = filename.as_ref().is_some();

    html! {
        <>
        <SimpleField label="Quiz Title" help={form.error("title")} help_color={Color::Danger}>
            <Input oninput={form.field(|x| &mut x.title)} value={title.clone()} placeholder={TITLE_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Description" help={form.error("description")} help_color={Color::Danger}>
            <Input oninput={form.field(|x| &mut x.description)} value={description.clone()} placeholder={DESCRIPTION_DEFAULT} />
        </SimpleField>

        <SimpleField label="Explanation" help={EXPLANATION_HELP}>
            <Input oninput={form.field(|x| &mut x.explanation)} value={explanation.clone()} placeholder={EXPLANATION_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Image" help={form.error("image")} help_color={Color::Danger}>
            <File fullwidth={fullwidth} filename={filename} onupload={form.async_field(make_image)}/>
        </SimpleField>

        <Buttons>
            <Button color={Color::Info} outlined=true onclick={onback}>
                <Icon icon={Icons::ArrowLeft}/> <span> {"Back"} </span>
            </Button>
            <Button color={Color::Primary} light=true disabled={!form.can_submit()} onclick={form.submit()}>
                <Icon icon={Icons::ArrowRight}/> <span> {"Rounds"} </span>
            </Button>
        </Buttons>
        </>
    }
}
