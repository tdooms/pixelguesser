use std::rc::Rc;

use cobul::*;
use yew::suspense::use_future_with_deps;
use yew::*;
use yew_router::hooks::use_navigator;

use api::Quiz;
use components::QuizCard;
use shared::callback;
use shared::{use_auth, use_toast, Route};

use crate::navbar::MainNavbar;
use crate::search::{Search, Sort};

#[derive(Properties, PartialEq, Clone)]
pub struct ColumnProps {
    quiz: Rc<Quiz>,
}

#[derive(Properties, PartialEq)]
pub struct InnerProps {
    pub filter: String,
}

async fn query(token: Option<String>, filter: String) -> Vec<Rc<Quiz>> {
    let quizzes: Vec<_> = match filter.is_empty() {
        true => Quiz::query_many(token, None, false).await.unwrap(),
        false => Quiz::search(token, filter).await.unwrap(),
    };

    quizzes.into_iter().map(Rc::new).collect()
}

#[function_component(QuizColumn)]
pub fn quiz_column(ColumnProps { quiz }: &ColumnProps) -> Html {
    let quiz_id = quiz.id.unwrap();
    let navigator = use_navigator().unwrap();

    let edit = callback!(navigator; move |_| navigator.push(&Route::Update{quiz_id}));
    let play = callback!(navigator; move |_| navigator.push(&Route::Host{quiz_id}));

    html! { <Column size={ColumnSize::Is3}> <QuizCard {quiz} {play} {edit} /> </Column> }
}

#[function_component(EmptyColumn)]
pub fn empty_column() -> Html {
    html! { <Column size={ColumnSize::Is3}> <QuizCard /> </Column> }
}

#[function_component(InnerOverview)]
pub fn inner_overview(props: &InnerProps) -> HtmlResult {
    let token = use_auth().token().map(|x| (*x).clone());
    // let toasts = use_toast();

    let fut = |filter: Rc<String>| async move { query(token, (*filter).clone()).await };

    let quizzes = use_future_with_deps(fut, props.filter.clone())?;
    Ok(html! { for quizzes.iter().cloned().map(|quiz| html!{ <QuizColumn {quiz} />}) })
}

#[function_component(Overview)]
pub fn overview() -> Html {
    let filter = use_model(|| String::new());
    let sort = use_model(|| Sort::Relevance);

    let fallback = html! { for (0..8).map(|_| html!{ <EmptyColumn /> }) };
    let inner = html! { <InnerOverview filter={filter.value.clone()} /> };

    html! {
        <>
        <MainNavbar/>
        <Section class="pt-0">
        <Container>
            <Search {sort} {filter} />
            <Columns multiline=true> <Suspense {fallback}> {inner} </Suspense> </Columns>
        </Container>
        </Section>
        </>
    }
}
