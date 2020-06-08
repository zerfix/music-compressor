// std
use std::sync::mpsc::Receiver;
// local
use super::super::util::config::Config;
use super::super::util::file::{file_copy, file_remove, file_rename};
use super::super::util::folders::inputpath_to_outputpath;
use super::Actions;


//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
/// this thread does the compression
pub fn compressor(rx: Receiver<Actions>, config: Config) {
    for e in rx {
        match e {
            Actions::Copy(path) => file_copy(
                path.clone(),
                inputpath_to_outputpath(path, &config.input_dir, &config.output_dir),
            ),
            Actions::Remove(path) => file_remove(inputpath_to_outputpath(
                path,
                &config.input_dir,
                &config.output_dir,
            )),
            Actions::Rename(old, new) => file_rename(
                old,
                new.clone(),
                inputpath_to_outputpath(new, &config.input_dir, &config.output_dir),
            ),
        }
    }
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
