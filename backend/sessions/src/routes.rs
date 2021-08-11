use rocket::response::stream::TextStream;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::to_string;
use rocket::State;
use tokio::sync::watch;

use shared::{Error, Session};

use crate::state::{SessionData, Sessions};
use crate::util::generate_id;

type Response<T> = Result<Json<T>, Json<Error>>;

#[post("/create")]
pub async fn create(sessions: &State<Sessions>) -> Response<(u64, Session)> {
    static SESSION_MAX: u64 = 48u64.pow(6);

    let mut lock = sessions.write().await;

    let validator = |id: u64| !lock.contains_key(&id);
    let id = generate_id(validator, 0..SESSION_MAX)
        .ok_or(Json(Error::SessionCreationFailed))?;

    let (sender, receiver) = watch::channel(Session::default());
    lock.insert(id, SessionData { sender, receiver });

    Ok(Json((id, Session::default())))
}

#[put("/<session_id>", data= "<session>")]
pub async fn update(sessions: &State<Sessions>, session_id: u64, session: Json<Session>) -> Response<()> {
    let lock = sessions.write().await;

    let _ = lock.get(&session_id)
        .ok_or(Json(Error::SessionDoesNotExist(session_id)))?
        .sender
        .send(session.into_inner());

    Ok(Json(()))
}

#[get("/<session_id>/check")]
pub async fn check(sessions: &State<Sessions>, session_id: u64) -> Response<bool> {
    Ok(Json(sessions.read().await.contains_key(&session_id)))
}

#[get("/<session_id>")]
pub async fn read(sessions: &State<Sessions>, session_id: u64) -> Response<Session> {
    match sessions.read().await.get(&session_id) {
        None => Err(Json(Error::SessionDoesNotExist(session_id))),
        Some(SessionData { receiver, .. }) => Ok(Json(receiver.borrow().clone()))
    }
}

// #[post("/<session_id>")]
// pub async fn create_player(sessions: &State<Sessions>, session_id: u64, name: String) -> Response<Player> {
//     match sessions.read().await.get(&session_id) {
//         None => Err(Json(Error::SessionDoesNotExist(session_id))),
//         Some(SessionData { receiver, sender }) => {
//             let mut cloned = receiver.borrow().clone();
//
//             match cloned.players.iter().find(|x| x.name == name) {
//                 None => {
//                     let player = Player { name, score: 0 };
//                     cloned.players.push(player.clone());
//
//                     let _ = sender.send(cloned);
//
//                     Ok(Json(player))
//                 }
//                 Some(_) => Err(Json(Error::DuplicatePlayerName(name)))
//             }
//         }
//     }
// }

#[get("/<session_id>/subscribe")]
pub async fn subscribe(sessions: &State<Sessions>, session_id: u64) -> TextStream![String] {
    let mut watcher = sessions
        .read()
        .await
        .get(&session_id)
        .unwrap()
        .receiver
        .clone();

    TextStream! {
        while let Ok(_) = watcher.changed().await {
            // This needs to be a separate line for lifetime/send/sync reasons
            let str = to_string(&* watcher.borrow()).unwrap();
            yield str;
        }
    }
}