use api::{Action, FullQuiz, Request, Response, Session, SESSION_ENDPOINT};
use shared::{use_full_quiz, Errors};
use std::rc::Rc;
use yew::prelude::*;
use yew_hooks::use_web_socket;

use cobul::Loading;
use host::Host;
use manage::Manage;

#[derive(Properties, Clone, Debug, PartialEq, Copy)]
pub struct Props {
    // Having a session_id implies being a manager
    pub session_id: Option<u64>,
    pub quiz_id: u64,
}

#[function_component(Loader)]
pub fn loader(props: &Props) -> HtmlResult {
    let Props { quiz_id, session_id } = props.clone();

    let ws = use_web_socket(SESSION_ENDPOINT);
    let session = use_state(|| None);
    let full = use_full_quiz(quiz_id)?;
    let errors = use_context::<Errors>().unwrap();

    let callback = {
        let ws = ws.clone();
        let rounds = full.as_ref().unwrap().rounds.len();

        Callback::from(move |action: Action| {
            let message = serde_json::to_string(Request::Update(action, rounds));
            ws.send(message);
        })
    };

    use_effect_with_deps(
        move |message| {
            if let Some(message) = &**message {
                let result: Result<Response, api::Error> = serde_json::from_str(&message).unwrap();
                match result {
                    Ok(Response { id, managed: _, session }) => {
                        // TODO: check if session with other id exists
                        session.set(Some((id, Rc::new(session))));
                    }
                    Err(err) => {
                        // TODO
                    }
                }
            }
            || ()
        },
        ws.message,
    );

    match (session.clone(), full.clone(), session_id) {
        (Some((_, session)), Some(full), Some(_)) => html! {
            <Manage {session} {full} {callback}/>
        },
        (Some((session_id, session)), Some(full), None) => html! {
            <Host {session} {session_id} {full} />
        },
        _ => html! {
            <Loading />
        },
    }
}
