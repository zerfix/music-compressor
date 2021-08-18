use std::sync::mpsc::{channel, Sender};
use std::time::Duration;

use notify::DebouncedEvent;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;

use crate::types::com::Actions;
use crate::types::config::Config;

//-////////////////////////////////////////////////////////////////////////////
//  File system listener
//-////////////////////////////////////////////////////////////////////////////
/// Listens for changes in input folder.
pub fn listen(config: Config, comp_tx: Sender<Actions>) {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();
    watcher.watch(config.dir.inn, RecursiveMode::Recursive).unwrap();

    for event in rx {
        match match event {
            DebouncedEvent::Create(path)       => Some(Actions::Copy(path)),
            DebouncedEvent::Write(path)        => Some(Actions::Copy(path)),
            DebouncedEvent::NoticeWrite(path)  => Some(Actions::Copy(path)),
            DebouncedEvent::Rename(prev, new)  => Some(Actions::Rename(prev, new)),
            DebouncedEvent::Remove(path)       => Some(Actions::Remove(path)),
            DebouncedEvent::NoticeRemove(path) => Some(Actions::Remove(path)),
            DebouncedEvent::Chmod(_)           => None,
            DebouncedEvent::Error(err, path)   => {eprintln!("Watch error at {:?}! {:?}", path, err); None }
            DebouncedEvent::Rescan             => {eprintln!("Unhadled rescan event!"); None }
        } {
            Some(s) => comp_tx.send(s).unwrap(),
            None => (),
        };
    }
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
