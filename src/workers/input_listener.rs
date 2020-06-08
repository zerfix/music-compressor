// std
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
// lib
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Result, Watcher};
// local
use super::super::util::config::Config;

use super::Actions;


//-////////////////////////////////////////////////////////////////////////////
// listener syncronizer
//-////////////////////////////////////////////////////////////////////////////
pub fn listen(config: Config, comp_tx: Sender<Actions>) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher
        .watch(config.input_dir, RecursiveMode::Recursive)
        .unwrap();

    loop {
        let msg = match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(path)       => Some(Actions::Copy(path.to_str().unwrap().to_string())),
                DebouncedEvent::Write(path)        => Some(Actions::Copy(path.to_str().unwrap().to_string())),
                DebouncedEvent::NoticeWrite(path)  => Some(Actions::Copy(path.to_str().unwrap().to_string())),
                DebouncedEvent::Rename(prev, new)  => Some(Actions::Rename(prev.to_str().unwrap().to_string(), new.to_str().unwrap().to_string())),
                DebouncedEvent::Remove(path)       => Some(Actions::Remove(path.to_str().unwrap().to_string())),
                DebouncedEvent::NoticeRemove(path) => Some(Actions::Remove(path.to_str().unwrap().to_string())),
                DebouncedEvent::Chmod(_)           => None,
                DebouncedEvent::Error(err, path)   => {println!("Watch error at {:?}! {:?}", path, err); None }
                DebouncedEvent::Rescan             => {println!("Unhadled rescan event!"); None }
            },
            Err(e) => { println!("input watch error: {:?}", e); None }
        };
        if let Some(s) = msg { comp_tx.send(s).unwrap() };
    }
}
