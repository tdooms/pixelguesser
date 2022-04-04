use cobul::props::{ColumnSize, Size};
use cobul::{Column, Columns, Container, Control, EnumDropdown, Field, Input, Section};
use strum::EnumIter;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
use yew::HtmlResult;

use api::{Quiz, Resolution, User};
use components::{EmptyCard, MainNavbar, QuizCard};
use shared::{callback, Auth};

#[derive(Clone, Copy, Debug, derive_more::Display, PartialEq, EnumIter)]
pub enum Sort {
    Relevance,
    Difficulty,
    Rating,
    Popularity,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    sort: Sort,
    filter: String,

    onsort: Callback<Sort>,
    onfilter: Callback<String>,
}

#[function_component(Search)]
pub fn search(props: &Props) -> Html {
    let Props { sort, filter, onsort, onfilter } = props.clone();

    html! {
        <Columns centered=true class="py-4">
        <Column size={ColumnSize::Is8}>

        <Field grouped=true>
            <Control expanded=true>
                <Input placeholder="Find a quiz" value={filter} oninput={onfilter} size={Size::Large}/>
            </Control>
            <Control>
                <EnumDropdown<Sort> size={Size::Large} value={sort} onchange={onsort}/>
            </Control>
        </Field>

        </Column>
        </Columns>
    }
}

#[function_component(Overview)]
pub fn overview() -> HtmlResult {
    let user = use_context::<Auth>().unwrap().user().ok();
    let filter = use_state_eq(|| String::new());
    let sort = use_state_eq(|| Sort::Relevance);

    let quizzes = use_state_eq(|| vec![]);
    let cloned = quizzes.clone();

    use_effect_with_deps(
        |deps| {
            let filter = (**deps).clone();
            spawn_local(async move {
                let result = match filter.is_empty() {
                    true => api::quizzes(user).await.unwrap(),
                    false => api::search_quizzes(user, filter).await.unwrap(),
                };
                cloned.set(result);
            });
            || ()
        },
        filter.clone(),
    );

    let onfilter = callback!(filter; move |x| filter.set(x));
    let onsort = callback!(sort; move |x| sort.set(x));

    let view_quiz_card = |quiz: &Quiz| {
        let Quiz { title, description, image, id, creator, public, .. } = quiz.clone();

        let image = api::Image::src_or_placeholder(
            image.map(|url| api::Image::from_url(url, String::new())).as_ref(),
            Resolution::Card,
        );

        html! {
            <Column size={ColumnSize::Is3}>
                <QuizCard {id} {title} {description} {image} {creator} {public}/>
            </Column>
        }
    };

    let view_empty_card = |_| html! {<Column size={ColumnSize::Is3}><EmptyCard/></Column>};

    let list = match quizzes.len() {
        0 => html! { for (0..8).map(view_empty_card) },
        _ => html! { for quizzes.iter().map(view_quiz_card) },
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
