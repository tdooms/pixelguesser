use crate::lib::Session;
use crate::{Global, Local, SinkExt, State, StreamExt};
use axum::extract::ws::{Message, WebSocket};
use pixessions::{Action, Error};
use rand::Rng;

async fn handle_message(message: Message, state: &mut State, conn_id: u32) -> Result<(), Error> {
    let message = message.into_text().map_err(|_| Error::NonText)?;
    let action: Action = serde_json::from_str(&message)?;

    state.session.update(action, conn_id)?;
    notify(state, &state.session.clone()).await;

    Ok(())
}

async fn handle_local(global: &Global, session_id: u32) -> Result<Local, Error> {
    let lock = global.lock().await;

    Ok(lock.get(&session_id).cloned().ok_or(Error::SessionNotFound)?)
}

async fn notify(state: &mut State, session: &Session) {
    let response = serde_json::to_string(session).unwrap();
    for (_, sender) in &mut state.connections {
        let _ = sender.send(Message::Text(response.clone())).await;
    }
}

async fn handle_session(stream: WebSocket, global: Global, session_id: u32) {
    let (sender, mut receiver) = stream.split();
    let conn_id = rand::thread_rng().gen::<u32>();
    log::info!("attempted connection to {session_id}");

    // Add the sender to the connections of the local state
    let local = match handle_local(&global, session_id).await.map_err(|e| log::error!("{e}")) {
        Ok(local) => local,
        Err(_) => return,
    };

    local.lock().await.connections.insert(conn_id, sender);

    while let Some(Ok(message)) = receiver.next().await {
        let mut lock = local.lock().await;
        if let Err(err) = handle_message(message, &mut *lock, conn_id).await {
            log::error!("{err}");
        }
    }

    // Remove local connection from the session
    let mut lock = local.lock().await;
    lock.connections.remove(&conn_id);

    // Remove the session from the global state if it has no more connections
    if lock.connections.is_empty() {
        global.lock().await.remove(&session_id);
    }

    log::debug!("{:?}", lock.session.participants);

    // Remove the participant corresponding to this connection
    // TODO: Should participants be part of local or session?
    lock.session.participants.retain(|_, v| *v != conn_id);

    log::debug!("{:?}", lock.session.participants);

    // Notify the rest of the participants of the session change
    let session = lock.session.clone();
    notify(&mut *lock, &session).await;
}
