use crate::{constants, debug, error};
use notify::Watcher;
use std::sync::{LazyLock, RwLock};

#[allow(dead_code)]
pub struct WatcherState {
    pub tx: crossbeam::channel::Sender<notify::Result<notify::Event>>,
    pub rx: crossbeam::channel::Receiver<notify::Result<notify::Event>>,
    pub watcher: notify::INotifyWatcher,
}
impl WatcherState {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam::channel::unbounded::<notify::Result<notify::Event>>();
        let watcher = notify::recommended_watcher(tx.clone()).unwrap();
        WatcherState { tx, rx, watcher }
    }
}
pub(crate) static WATCHER_STATE: LazyLock<RwLock<Option<WatcherState>>> =
    LazyLock::new(|| RwLock::new(None));

pub fn watcher_run(some_path: String) -> Result<(), error::Error> {
    let mut state = WatcherState::new();
    state
        .watcher
        .watch(
            &std::path::Path::new(&some_path),
            notify::RecursiveMode::Recursive,
        )
        .unwrap();

    let mut watcher_state = WATCHER_STATE.write()?;
    match *watcher_state {
        Some(_) => {
            return Err(error::Error::StateAlreadyInitializedError);
        }
        None => {
            *watcher_state = Some(state);
        }
    }

    Ok(())
}

pub fn watcher_poll_events() -> Result<(), error::Error> {
    let mut write_guard = WATCHER_STATE.write()?;
    let watcher_state = write_guard
        .as_mut()
        .ok_or(error::Error::StateNotInitializedError)?;

    for _ in 0..constants::MAX_WATCHER_EVENTS_PER_FRAME {
        match watcher_state.rx.try_recv() {
            Ok(res) => match res {
                Ok(event) => match event.kind {
                    notify::EventKind::Create(_) => {
                        debug::info(&format!("create_event: {:#?}", event));
                    }
                    notify::EventKind::Remove(_) => {
                        debug::info(&format!("remove: {:#?}", event));
                    }
                    _ => {
                        debug::info("some other event");
                    }
                },
                Err(e) => return Err(error::Error::WatcherError(e.to_string())),
            },
            Err(crossbeam::channel::TryRecvError::Empty) => {
                break;
            }
            Err(crossbeam::channel::TryRecvError::Disconnected) => {
                return Err(error::Error::ChannelReceiveError(
                    "Failed to receive watcher events, channel is closed".to_string(),
                ));
            }
        }
    }

    Ok(())
}
