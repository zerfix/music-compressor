use std::sync::mpsc::Receiver;

use crate::CONF;
use crate::actions::file_system::{copy, remove, rename};
use crate::converters::path::input_to_output;
use crate::types::com::Actions;

//-////////////////////////////////////////////////////////////////////////////
//  Compression Listener
//-////////////////////////////////////////////////////////////////////////////
/// Handles changes in input filesystem.
pub fn compress(rx: Receiver<Actions>) {
    for action in rx {
        let res = match action {
            Actions::Copy(path) => copy(
                path.clone(),
                input_to_output(&path, &CONF.dir)
            ),
            Actions::Remove(path) => remove(input_to_output(&path, &CONF.dir)),
            Actions::Rename(old, new) => rename(
                old,
                new.clone(),
                input_to_output(&new, &CONF.dir),
            ),
        };
        if let Ok(res) = res {
            println!("{}", res);
        } else {
            res.unwrap();
        }
    }
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
