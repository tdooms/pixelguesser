use cobul::Loader;
use std::rc::Rc;
use yew::*;

use host::Host;
use manage::Manage;

use api::{Action, Participant, Quiz, Result, Session, User, WebsocketTask};
use shared::use_auth;

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

async fn create_session(props: &Props, user: Option<Rc<User>>) -> Result<State> {
    let (session_id, participant) = match props.session_id {
        Some(id) => (id, Participant::Manager),
        None => (api::create_session(props.quiz_id).await?, Participant::Host),
    };
    let quiz = Rc::new(api::query_quiz(user, props.quiz_id).await?);

    Ok(State { session_id, participant, quiz })
}

#[function_component(Initializer)]
pub fn initializer(props: &Props) -> Html {
    let websocket = use_state(|| None);
    let state = use_state(|| None);
    let session = use_state(|| Rc::new(Session::default()));

    let cloned = session.clone();
    let callback = move |response| match response {
        Ok(new) => cloned.set(Rc::new(new)),
        Err(err) => log::error!("{err}"),
    };

    let user = use_auth().user();
    let (props_c, ws_c, state_c) = (props.clone(), websocket.clone(), state.clone());

    use_effect_with_deps(
        move |_| {
            ywt::spawn!(async move {
                let res = create_session(&props_c, user).await.unwrap();

                let task = WebsocketTask::new(res.session_id, callback).unwrap();
                task.send(&Action::Join(res.participant.clone()));

                ws_c.set(Some(task));
                state_c.set(Some(res));
            });
            || ()
        },
        (),
    );

    let callback = ywt::callback!(websocket; move |action| {
        websocket.as_ref().unwrap().send(&action)
    });
    let session = (*session).clone();

    match (props.session_id, (*state).clone()) {
        (Some(_), Some(State { quiz, .. })) => html! {
            <Manage {session} {quiz} {callback}/>
        },
        (None, Some(State { session_id, quiz, .. })) => html! {
            <Host {session_id} {session} {quiz} {callback}/>
        },
        _ => html! { <Loader/> },
    }
}
