use crate::session::{Sender, Session};
use api::Stage;
use rand::Rng;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

static MAX: u64 = 48u64.pow(6);

#[derive(Default)]
pub struct Inner {
    sessions: Mutex<HashMap<u64, Session>>,
}

#[derive(Clone)]
pub struct State {
    pool: PgPool,
    inner: Arc<Inner>,
}

impl State {
    pub async fn new(uri: &str) -> Result<Self, sqlx::Error> {
        log::info!("Connecting to database with uri {}", uri);

        Ok(Self {
            pool: PgPool::connect(uri).await?,
            inner: Arc::default(),
        })
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub async fn sessions(&self) -> MutexGuard<'_, HashMap<u64, Session>> {
        self.inner.sessions.lock().await
    }

    pub async fn start_session(&self, sender: Sender, quiz_id: i64, rounds: usize) -> Option<u64> {
        let session = Session {
            host: sender,
            manager: None,
            rounds,
            quiz_id,
            stage: Stage::Initial,
            players: HashMap::new(),
            current_id: 0,
        };

        let mut lock = self.sessions().await;

        // Generate random ID's and try to make a session
        for _ in 0..10 {
            let id = rand::thread_rng().gen_range(0..MAX);
            // Use entry API to avoid duplicate search?
            if !lock.contains_key(&id) {
                lock.insert(id, session);
                return Some(id);
            }
        }

        None
    }
}
