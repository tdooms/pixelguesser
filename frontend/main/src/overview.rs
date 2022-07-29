use crate::navbar::MainNavbar;
use crate::search::{Search, Sort};
use api::Quiz;
use cobul::*;
use components::{QuizCard, View};
use shared::{Auth, EmitError, Errors, Route};
use wasm_bindgen_futures::spawn_local;
use yew::HtmlResult;
use yew::*;
use yew_router::hooks::use_navigator;
use ywt::callback;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    quiz: Quiz,
}

#[function_component(QuizColumn)]
pub fn quiz_column(props: &Props) -> Html {
    let quiz = props.quiz.clone();
    let quiz_id = quiz.id;

    let navigator = use_navigator().unwrap();

    let onedit = callback!(navigator; move |_| {
        navigator.push(Route::Update{quiz_id})
    });
    let onclick = callback!(navigator; move |_| {
        navigator.push(Route::Host{quiz_id})
    });

    html! {
        <Column size={ColumnSize::Is3}>
            <QuizCard view={View::Normal{quiz, onclick, onedit}}/>
        </Column>
    }
}

#[function_component(EmptyColumn)]
pub fn empty_column() -> Html {
    html! {<Column size={ColumnSize::Is3}><QuizCard view={View::Empty}/></Column>}
}

#[function_component(Overview)]
pub fn overview() -> HtmlResult {
    let user = use_context::<Auth>().unwrap().user().ok();
    let errors = use_context::<Errors>().unwrap();
    let filter = use_state_eq(|| String::new());
    let sort = use_state_eq(|| Sort::Relevance);

    let quizzes = use_state_eq(|| None);
    let cloned = quizzes.clone();

    use_effect_with_deps(
        move |deps| {
            let filter = (**deps).clone();
            spawn_local(async move {
                let result = match filter.is_empty() {
                    true => api::query_quizzes(user, false).await,
                    false => api::search_quizzes(user, filter, false).await,
                };
                cloned.set(result.emit(&errors));
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
        <Section class="pt-0">
        <Container>
            <Search {onsort} {onfilter} sort={*sort} filter={(*filter).clone()}/>
            <Columns multiline=true> {list} </Columns>
        </Container>
        </Section>
        </>
    })
}
