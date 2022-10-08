use api::query_sessions;
use api::Session;
use cobul::*;
use shared::{use_toast, Route};
use std::ops::Deref;
use yew::suspense::use_future;
use yew::*;
use yew_router::prelude::use_navigator;

#[function_component(Sessions)]
pub fn sessions() -> HtmlResult {
    let future = use_future(query_sessions)?;
    let toast = use_toast();
    let navigator = use_navigator().unwrap();

    let mapper = |session: &Session| {
        html! { <Box>{ session.quiz.to_string() }</Box> }
    };

    let sessions = future.as_ref().unwrap();
    let list = html! { for sessions.iter().map(mapper) };

    Ok(html! {
        <Section class="pt-0">
        <Container>
            <Columns multiline=true> {list} </Columns>
        </Container>
        </Section>
    })
}
