use crate::navbar::MainNavbar;
use crate::search::{Search, Sort};
use api::{Quiz, Resolution};
use cobul::*;
use components::{EmptyCard, QuizCard};
use shared::{callback, Auth};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::HtmlResult;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    quiz: Quiz,
}

#[function_component(QuizColumn)]
pub fn quiz_column(Props { quiz }: &Props) -> Html {
    let Quiz { title, description, image, id, creator, public, .. } = quiz.clone();

    let image = api::Image::src_or_placeholder(
        image.map(|url| api::Image::from_url(url, String::new())).as_ref(),
        Resolution::Card,
    );

    html! {
        <Column size={ColumnSize::Is3}>
            <QuizCard id={id as u32} {title} {description} {image} {creator} {public}/>
        </Column>
    }
}

#[function_component(EmptyColumn)]
pub fn empty_column() -> Html {
    html! {<Column size={ColumnSize::Is3}><EmptyCard/></Column>}
}

#[function_component(Overview)]
pub fn overview() -> HtmlResult {
    let user = use_context::<Auth>().unwrap().user().ok();
    let filter = use_state_eq(|| String::new());
    let sort = use_state_eq(|| Sort::Relevance);

    let quizzes = use_state_eq(|| None);
    let cloned = quizzes.clone();

    use_effect_with_deps(
        |deps| {
            let filter = (**deps).clone();
            spawn_local(async move {
                let result = match filter.is_empty() {
                    true => api::quizzes(user).await.unwrap(),
                    false => api::search_quizzes(user, filter).await.unwrap(),
                };
                cloned.set(Some(result));
            });
            || ()
        },
        filter.clone(),
    );

    let onfilter = callback!(filter; move |x| filter.set(x));
    let onsort = callback!(sort; move |x| sort.set(x));

    let list = match &*quizzes {
        None => html! { for (0..8).map(|_| html!{<EmptyColumn/>}) },
        Some(all) => html! { for all.iter().cloned().map(|quiz| html!{ <QuizColumn {quiz}/>}) },
    };

    Ok(html! {
        <>
        <MainNavbar/>
        <Section>
        <Container>
            <Search {onsort} {onfilter} sort={*sort} filter={(*filter).clone()}/>
            <Columns multiline=true> {list} </Columns>
        </Container>
        </Section>
        </>
    })
}
