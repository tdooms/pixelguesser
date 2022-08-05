use cobul::Loader;
use std::rc::Rc;
use yew::*;

use host::Host;
use manage::Manage;

use api::{Action, Participant, Quiz, Session, User, WebsocketTask};
use shared::{use_async_startup, use_auth, Kind, Toast};

#[derive(derive_more::Display, Clone, Copy)]
pub enum Error {
    #[display(fmt = "Unable to start a quiz, please try again later")]
    Unreachable,
}

impl Toast for Error {
    fn kind(&self) -> Kind {
        Kind::Error
    }

    fn leave(&self) -> bool {
        true
    }
}

#[derive(Properties, Clone, Debug, PartialEq, Copy)]
pub struct Props {
    pub session_id: Option<u32>, // Having a session_id implies being a manager
    pub quiz_id: u32,
}

#[derive(Clone)]
struct State {
    session_id: u32,
    participant: Participant,
    quiz: Rc<Quiz>,
}

async fn create_session(props: &Props, user: Option<Rc<User>>) -> Result<State, api::Error> {
    let (session_id, participant) = match props.session_id {
        Some(id) => (id, Participant::Manager),
        None => (api::create_session(props.quiz_id).await?, Participant::Host),
    };
    let quiz = Rc::new(api::query_quiz(user, props.quiz_id).await?);

    Ok(State { session_id, participant, quiz })
}

async fn init(
    ws: UseStateHandle<Option<WebsocketTask>>,
    state: UseStateHandle<Option<State>>,
    session: UseStateHandle<Option<Rc<Session>>>,
    props: Props,
    user: Option<Rc<User>>,
) -> Result<(), Error> {
    let mapper = |_| Error::Unreachable;

    let callback = move |response| match response {
        Ok(new) => session.set(Some(Rc::new(new))),
        Err(err) => log::error!("{err}"),
    };

    let created = create_session(&props, user).await.map_err(mapper)?;
    let task = WebsocketTask::new(created.session_id, callback).map_err(mapper)?;

    task.send(&Action::Join(created.participant.clone()));

    ws.set(Some(task));
    state.set(Some(created));
    Ok(())
}

#[function_component(Initializer)]
pub fn initializer(props: &Props) -> Html {
    let websocket = use_state(|| None);
    let state = use_state(|| None);
    let session = use_state(|| None);
    let user = use_auth().user();

    use_async_startup(init(websocket.clone(), state.clone(), session.clone(), props.clone(), user));

    let callback = ywt::callback!(websocket; move |action| {
        websocket.as_ref().unwrap().send(&action)
    });

    match (props.session_id, (*state).clone(), (*session).clone()) {
        (Some(_), Some(State { quiz, .. }), Some(session)) => html! {
            <Manage {session} {quiz} {callback}/>
        },
        (None, Some(State { session_id, quiz, .. }), Some(session)) => html! {
            <Host {session_id} {session} {quiz} {callback}/>
        },
        _ => html! { <Loader/> },
    }
}
