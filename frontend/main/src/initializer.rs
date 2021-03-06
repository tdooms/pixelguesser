use std::rc::Rc;

use cobul::Loader;
use yew::*;

use yew_router::prelude::RouterScopeExt;
use ywt::spawn;

use host::Host;
use manage::Manage;

use api::{Action, Participant, Quiz, Response, Session, WebsocketTask};
use shared::{Auth, EmitError, Error, Errors, Route};

#[derive(Properties, Clone, Debug, PartialEq, Copy)]
pub struct Props {
    // Having a session_id implies being a manager
    pub session_id: Option<u32>,
    pub quiz_id: u32,
}

pub struct Initializer {
    ws: Option<WebsocketTask>,
    session: Rc<Session>,
    quiz: Option<Rc<Quiz>>,

    session_id: Option<u32>,
}

pub enum Msg {
    Ws(Response),
    Quiz(api::Result<Quiz>),
    Session(Result<u32, api::Error>),
    Action(Action),
}

impl Component for Initializer {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let Props { session_id, quiz_id } = ctx.props().clone();

        match session_id {
            None => {
                let callback = ctx.link().callback(Msg::Session);
                spawn!(async move {
                    let result = api::create_session(quiz_id).await;
                    callback.emit(result);
                });
            }
            Some(id) => ctx.link().send_message(Msg::Session(Ok(id))),
        }

        let (auth, _) = ctx.link().context::<Auth>(Callback::noop()).unwrap();

        let callback = ctx.link().callback(Msg::Quiz);
        spawn!(async move {
            let result = api::query_quiz(auth.user().ok(), quiz_id).await;
            callback.emit(result);
        });

        Self { ws: None, session_id: None, session: Rc::new(Session::default()), quiz: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (errors, _) = ctx.link().context::<Errors>(Callback::noop()).unwrap();
        match msg {
            Msg::Quiz(Ok(quiz)) => {
                // Check if this only happens once?
                self.quiz = Some(Rc::new(quiz))
            }
            Msg::Action(action) => self.ws.as_mut().unwrap().send(&action),
            Msg::Ws(Response::Update(session)) => {
                self.session = Rc::new(session);
            }
            Msg::Quiz(Err(err)) => {
                errors.emit(Error::Api(err));
                ctx.link().navigator().unwrap().push(Route::Overview)
            }
            Msg::Ws(Response::Error(err)) => {
                log::error!("{:?}", err)
            }
            Msg::Session(Err(err)) => {
                log::error!("{:?}", err)
            }
            Msg::Session(Ok(session_id)) => {
                let cb = ctx.link().callback(Msg::Ws);
                let mut ws = WebsocketTask::new(session_id, cb).emit(&errors);

                let participant = match ctx.props().session_id {
                    None => Participant::Host,
                    Some(_) => Participant::Manager,
                };

                if let Some(ws) = &mut ws {
                    ws.send(&Action::Join(participant));
                }

                self.session_id = Some(session_id);
                self.ws = ws;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Initializer { session, quiz, session_id, .. } = self;
        let callback = ctx.link().callback(Msg::Action);

        match (session_id, quiz.clone(), ctx.props().session_id) {
            (Some(_), Some(quiz), Some(_)) => html! {
                <Manage {session} {quiz} {callback}/>
            },
            (Some(session_id), Some(quiz), None) => html! {
                <Host {session} {session_id} {quiz} {callback}/>
            },
            _ => html! {
                <Loader />
            },
        }
    }
}
