use bytes::Bytes;
use std::sync::{Arc, Mutex};
use std::collections::{HashMap, BTreeMap};
use std::time::{self, Duration, Instant};
use tokio::sync::Notify;

#[derive(Debug, Clone)]
pub(crate) struct Db {
    shared: Arc<Shared>,
}

impl Db {
    pub(crate) fn new() -> Db {
        let shared = Arc::new(Shared{
                state: Mutex::new(State {
                    entries: HashMap::new(),
                    expirations: BTreeMap::new(), 
                    next_id: 0,
            }),
            background_task: Notify::new(),

        });
        Db { shared }
    }

    fn clone() {

    }
}

#[derive(Debug)]
pub(crate) struct DbDropGuard {
    db: Db,
}

impl DbDropGuard {
    pub(crate) fn new() -> DbDropGuard {
        DbDropGuard { db: Db::new() }
    }

    pub(crate) fn db(&self) -> Db {
        self.db.clone()
    }
}

impl Db {
    pub(crate) fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock().unwrap();
        state.entries.get(key).map(|entry| entry.data.clone())
    }

    pub(crate) fn set(&self, key: String, value: Bytes, expire: Option<Duration>) {
        let mut state = self.shared.state.lock().unwrap();

        let id = state.next_id;
        state.next_id += 1;
        
        let mut notify = false;
        let expires_at = expire.map(|duration| {
            let when = Instant::now() + duration;

            notify = state
                .next_expiration()
                .map(|expiration| expiration > when)
                .unwrap_or(true);

            state.expirations.insert((when, id), key.clone());
            when
        });

        let prev = state.entries.insert(
            key,
            Entry {
                id,
                data: value,
                expires_at,
            },
        );

        if let Some(prev) = prev {
            if let Some(when) = prev.expires_at {
                state.expirations.remove(&(when, prev.id));
            } 
        }

        drop(state);

        if notify {
            self.shared.background_task.notify_one();
        }
    }
}

#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
    background_task: Notify,
}

#[derive(Debug)]
struct State {
    entries: HashMap<String, Entry>, 
    expirations: BTreeMap<(Instant, u64), String>,
    next_id: u64,
}

impl State {
    fn next_expiration(&self) -> Option<Instant> {
        self.expirations
            .keys()
            .next()
            .map(|expiration| expiration.0)
    }
}

#[derive(Debug)]
struct Entry {
    data: Bytes,
    id: u64,
    expires_at: Option<Instant>,
}