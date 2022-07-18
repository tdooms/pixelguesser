use api::FullDraftQuiz;
use cobul::*;
use gloo_file::callbacks::read_as_text;
use yew::*;
use ywt::callback;

#[derive(serde::Deserialize)]
struct Quizzes {
    quizzes: Vec<FullDraftQuiz>,
}

#[function_component(Database)]
pub fn database() -> Html {
    let state = use_state(|| None);
    let onread = callback!(state; move |string: String| {
        let quizzes = serde_json::from_str::<Quizzes>(&string).unwrap();
        state.set(Some(quizzes))
    });
    let onupload = callback!(onread; move |files: Vec<web_sys::File>| {
        let blob = gloo_file::Blob::from(files[0].clone());
        let onread = onread.clone();
        read_as_text(&blob, move |x| onread.emit(x.unwrap()));
    });

    let body = match &*state {
        Some(contents) => html! {
            <>
            <Title> {"yay uploaded"} </Title>
            // {contents}
            </>
        },
        None => html! {
            <File accept="application/json" {onupload} />
        },
    };

    html! {
        <Section>
        {body}
        </Section>
    }
}
