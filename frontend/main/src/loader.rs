use std::rc::Rc;

use cobul::custom::Loading;
use yew::*;

use yew_router::prelude::RouterScopeExt;
use ywt::spawn;

use host::Host;
use manage::Manage;

use api::{Action, FullQuiz, Participant, Response, Session, WebsocketTask};
use shared::{Auth, Error, Errors, Route};

#[derive(Properties, Clone, Debug, PartialEq, Copy)]
pub struct Props {
    // Having a session_id implies being a manager
    pub session_id: Option<u32>,
    pub quiz_id: u32,
}

pub struct Loader {
    ws: Option<WebsocketTask>,
    session: Rc<Session>,
    full: Option<Rc<FullQuiz>>,

    session_id: Option<u32>,
}

pub enum Msg {
    Ws(Response),
    Quiz(Result<FullQuiz, api::Error>),
    Session(Result<u32, api::Error>),
    Action(Action),
}

impl Component for Loader {
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
            let result = api::full_quiz(auth.user().ok(), quiz_id).await;
            callback.emit(result);
        });

        Self { ws: None, session_id: None, session: Rc::new(Session::default()), full: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (errors, _) = ctx.link().context::<Errors>(Callback::noop()).unwrap();
        match msg {
            Msg::Quiz(Ok(full)) => {
                // Check if this only happens once?
                self.full = Some(Rc::new(full))
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
                let mut ws = WebsocketTask::new(session_id, cb);

                let participant = match ctx.props().session_id {
                    None => Participant::Host,
                    Some(_) => Participant::Manager,
                };

                ws.send(&Action::Join(participant));

                self.session_id = Some(session_id);
                self.ws = Some(ws);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Loader { session, full, session_id, .. } = self;
        let callback = ctx.link().callback(Msg::Action);

        match (session_id, full.clone(), ctx.props().session_id) {
            (Some(_), Some(full), Some(_)) => html! {
                <Manage {session} {full} {callback}/>
            },
            (Some(session_id), Some(full), None) => html! {
                <Host {session} {session_id} {full} {callback}/>
            },
            _ => html! {
                <Loading />
            },
        }
    }
}
