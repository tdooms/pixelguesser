use crate::navbar::MainNavbar;
use crate::search::{Search, Sort};
use api::Quiz;
use cobul::*;
use components::QuizCard;
use shared::{use_auth, use_toast, Route};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::HtmlResult;
use yew::*;
use yew_router::hooks::use_navigator;
use ywt::callback;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    quiz: Rc<Quiz>,
}

#[function_component(QuizColumn)]
pub fn quiz_column(Props { quiz }: &Props) -> Html {
    let quiz_id = quiz.id.unwrap();

    let navigator = use_navigator().unwrap();

    let edit = callback!(navigator; move |_| {
        navigator.push(Route::Update{quiz_id})
    });
    let play = callback!(navigator; move |_| {
        navigator.push(Route::Host{quiz_id})
    });

    html! {
        <Column size={ColumnSize::Is3}>
            <QuizCard {quiz} {play} {edit} />
        </Column>
    }
}

#[function_component(EmptyColumn)]
pub fn empty_column() -> Html {
    html! { <Column size={ColumnSize::Is3}> <QuizCard /> </Column> }
}

#[function_component(Overview)]
pub fn overview() -> HtmlResult {
    let token = use_auth().token().map(|x| (*x).clone());
    let toasts = use_toast();

    let filter = use_state_eq(|| String::new());
    let sort = use_state_eq(|| Sort::Relevance);

    let quizzes = use_state_eq(|| None);
    let cloned = quizzes.clone();

    use_effect_with_deps(
        move |deps| {
            let filter = (**deps).clone();
            spawn_local(async move {
                let result = match filter.is_empty() {
                    true => Quiz::query_many(token, false).await,
                    false => Quiz::search(token, filter).await,
                };
                if let Some(quizzes) = toasts.maybe(result) {
                    let rc: Vec<_> = quizzes.into_iter().map(Rc::new).collect();
                    cloned.set(Some(rc));
                }
            });
            || ()
        },
        filter.clone(),
    );

    let list = match &*quizzes {
        None => html! { for (0..8).map(|_| html!{ <EmptyColumn /> }) },
        Some(all) => html! { for all.iter().cloned().map(|quiz| html!{ <QuizColumn {quiz} />}) },
    };

    let sort = Model { input: callback!(sort; move |x| sort.set(x)), value: *sort };
    let filter =
        Model { input: callback!(filter; move |x| filter.set(x)), value: (*filter).clone() };

    Ok(html! {
        <>
        <MainNavbar/>
        <Section class="pt-0">
        <Container>
            <Search {sort} {filter} />
            <Columns multiline=true> {list} </Columns>
        </Container>
        </Section>
        </>
    })
}
