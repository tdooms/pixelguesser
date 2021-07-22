use std::collections::HashMap;

use log::{debug, warn};
use tokio::sync::MutexGuard;
use warp::ws::Message;

use api::{Alert, Fetch, Get, Player, Post, Reply, Request, Response, Session};

use crate::db::{quiz_and_rounds, quizzes};
use crate::error::Error;
use crate::session::{Sender, SessionData};
use crate::state::State;

fn send(response: Response, sender: &Sender) -> Result<(), Error> {
    let string = serde_json::to_string(&response)?;
    sender.send(Ok(Message::text(string)))?;
    Ok(())
}

fn notify(response: Response, host: &Sender, manager: Option<&Sender>) -> Result<(), Error> {
    if let Some(manager) = manager {
        send(response.clone(), manager)?;
    }
    send(response, host)
}

fn get_session<'a>(
    lock: &'a mut MutexGuard<HashMap<u64, SessionData>>,
    session_id: u64,
) -> Result<&'a mut SessionData, Error> {
    lock.get_mut(&session_id).ok_or(Error::SessionDoesNotExist(session_id))
}

async fn handle_get(get: Get, state: &State, sender: &Sender) -> Result<(), Error> {
    let fetch = match get {
        Get::Quizzes => {
            let quizzes = quizzes(state.pool()).await?;
            Fetch::Quizzes(quizzes)
        }
        Get::Quiz { quiz_id } => {
            let (quiz, rounds) = quiz_and_rounds(quiz_id, state.pool()).await?;
            Fetch::Quiz(quiz, rounds)
        }
        Get::Stage { session_id } => {
            let mut lock = state.sessions().await;
            let session = get_session(&mut lock, session_id)?;

            Fetch::Stage(session.stage.clone())
        }
        Get::Players { session_id } => {
            let mut lock = state.sessions().await;
            let session = get_session(&mut lock, session_id)?;

            Fetch::Players(session.players.clone())
        }
        Get::CheckSession { session_id } => {
            let lock = state.sessions().await;

            match lock.get(&session_id) {
                Some(SessionData { manager: None, .. }) => Fetch::SessionAvailable(session_id),
                _ => Fetch::SessionInvalid(session_id),
            }
        }
    };

    send(Response::Fetch(fetch), sender)
}

async fn handle_post(post: Post, state: &State, sender: &Sender) -> Result<(), Error> {
    match post {
        Post::StartSession { quiz_id } => {
            let (quiz, rounds) = quiz_and_rounds(quiz_id, state.pool()).await?;

            let session_id = state
                .start_session(sender.clone(), quiz_id, rounds.len())
                .await
                .ok_or(Error::SessionCreationFailed)?;

            let response = Response::Reply(session_id, Reply::SessionCreated(quiz, rounds));
            send(response, sender)
        }
        Post::JoinSession { session_id } => {
            let mut lock = state.sessions().await;
            let session = get_session(&mut lock, session_id)?;

            if let Some(_) = session.manager {
                let response = Response::Reply(session_id, Reply::SessionManaged);
                send(response, sender)?;
            }

            session.manager = Some(sender.clone());
            let (quiz, rounds) = quiz_and_rounds(session.quiz_id, state.pool()).await?;

            let data = Session {
                stage: session.stage.clone(),
                quiz,
                rounds,
                players: session.players.clone(),
            };

            let response = Response::Reply(session_id, Reply::SessionJoined(data));
            send(response, sender)?;

            let response = Response::Alert(session_id, Alert::ManagerJoined);
            send(response, &session.host)
        }
        Post::AddPlayer { session_id, name } => {
            let mut lock = state.sessions().await;
            let session = get_session(&mut lock, session_id)?;

            let player = Player { name: name.clone(), score: 0 };

            let player_id = session.current_id;
            session.current_id += 1;
            session.players.insert(player_id, player);

            let alert = Alert::PlayerAdded(player_id, name);
            let response = Response::Alert(session_id, alert);
            notify(response, &session.host, session.manager.as_ref())
        }
        Post::ChangeScores { session_id, diff } => {
            let mut lock = state.sessions().await;
            let session = get_session(&mut lock, session_id)?;

            for change in &diff {
                let error = Error::PlayerDoesNotExist(change.player_id);
                let player = session.players.get_mut(&change.player_id).ok_or(error)?;

                player.score += change.change
            }

            let alert = Alert::ScoreChanged(diff);
            let response = Response::Alert(session_id, alert);
            notify(response, &session.host, session.manager.as_ref())
        }
        Post::ChangeStage { session_id, stage } => {
            let mut lock = state.sessions().await;
            let session = get_session(&mut lock, session_id)?;

            // TODO: fix stage check for end of quiz
            session.stage = stage.clone();

            let alert = Alert::StageChanged(stage);
            let response = Response::Alert(session_id, alert);
            notify(response, &session.host, session.manager.as_ref())
        }
        Post::StopSession { session_id } => {
            let mut lock = state.sessions().await;

            if let Some(SessionData { manager: Some(manager), .. }) = lock.remove(&session_id) {
                let response = Response::Alert(session_id, Alert::SessionStopped);
                send(response, &manager)?;
            }

            Ok(())
        }
        Post::LeaveSession { session_id } => {
            let mut lock = state.sessions().await;
            let session = get_session(&mut lock, session_id)?;

            session.manager = None;

            let response = Response::Alert(session_id, Alert::ManagerLeft);
            send(response, &session.host)
        }
    }
}

pub async fn handle_request(request: &[u8], state: &State, sender: &Sender) {
    let result = match serde_json::from_slice(request) {
        Ok(Request::Get(get)) => handle_get(get, state, sender).await,
        Ok(Request::Post(post)) => handle_post(post, state, sender).await,
        Err(err) => Err(Error::DeserializeError(err)),
    };

    let option = match result {
        Err(Error::PlayerDoesNotExist(id)) => Some(api::Error::PlayerDoesNotExist(id)),
        Err(Error::SessionDoesNotExist(id)) => Some(api::Error::SessionDoesNotExist(id)),
        Err(Error::SessionCreationFailed) => Some(api::Error::SessionCreationFailed),
        Err(Error::WsError(_)) => Some(api::Error::InternalServerError),
        Err(Error::DatabaseError(_)) => Some(api::Error::InternalServerError),
        Err(Error::DeserializeError(_)) => Some(api::Error::FaultyRequest),
        Ok(()) => None,
    };

    match option {
        Some(error) => {
            warn!("error while handling request: {:?}", error);
            let _ = send(Response::Error(error), sender);
        }
        None => debug!("successfully handled request"),
    }
}
