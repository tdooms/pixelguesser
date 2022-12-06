use std::rc::Rc;

use cobul::{Color, Loader};
use yew::suspense::use_future;
use yew::*;

use api::{Action, Participant, Quiz, Session, WebsocketTask};
use host::Host;
use manage::Manage;
use shared::{callback, use_auth, use_toast, Level, Toast};

#[derive(Properties, Clone, Debug, PartialEq, Copy)]
pub struct Props {
    pub session_id: Option<u64>,
    // Having a session_id implies being a manager
    pub quiz_id: u64,
}

#[derive(Clone)]
struct State {
    session_id: u64,
    participant: Participant,
    quiz: Rc<Quiz>,
}

async fn create_session(props: &Props, token: Option<String>) -> Result<State, api::Error> {
    let (session_id, participant) = match props.session_id {
        Some(id) => (id, Participant::Manager),
        None => (api::create_session(props.quiz_id).await?, Participant::Host),
    };
    let quiz = Rc::new(Quiz::query_one(token, props.quiz_id, Some(session_id)).await?);

    Ok(State { session_id, participant, quiz })
}

async fn init(
    ws: UseStateHandle<Option<WebsocketTask>>,
    state: UseStateHandle<Option<State>>,
    session: UseStateHandle<Option<Rc<Session>>>,
    props: Props,
    token: Option<String>,
) -> Result<(), api::Error> {
    let callback = move |response| match response {
        Ok(new) => session.set(Some(Rc::new(new))),
        Err(err) => tracing::error!("{err}"),
    };

    let created = create_session(&props, token).await?;
    let task = WebsocketTask::new(created.session_id, callback)?;

    task.send(&Action::Join(created.participant.clone()));

    ws.set(Some(task));
    state.set(Some(created));
    Ok(())
}

#[function_component(Initializer)]
pub fn initializer(props: &Props) -> HtmlResult {
    let toast = use_toast();
    let ws = use_state(|| None);
    let state = use_state(|| None);
    let session = use_state(|| None);
    let token = use_auth().token().map(|x| (*x).clone());

    let fut = init(ws.clone(), state.clone(), session.clone(), props.clone(), token);
    use_future(|| async move { toast.api(fut.await) })?;

    let action = callback!(ws; move |action| ws.as_ref().unwrap().send(&action));

    Ok(match (props.session_id, (*state).clone(), (*session).clone()) {
        (Some(_), Some(State { quiz, .. }), Some(session)) => html! {
            <Manage {session} {quiz} {action} />
        },
        (None, Some(State { session_id, quiz, .. }), Some(session)) => html! {
            <Host {session_id} {session} {quiz} {action}/>
        },
        _ => html! {
            <Loader color={Color::Info} />
        },
    })
}
