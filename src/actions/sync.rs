use std::fs::{read_dir, DirEntry};
use std::io;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

use crate::actions::file_system::find_input;
use crate::converters::path::{input_to_output, output_to_input};
use crate::types::com::Actions;
use crate::types::config::Dir;

//-////////////////////////////////////////////////////////////////////////////
//  initial syncronization
//-////////////////////////////////////////////////////////////////////////////
/// Initial sync between input and output folder.
pub fn init_sync(tx: &Sender<Actions>, dir: &Dir) {
    let mut out_count = 0;
    let mut inn_count = 0;

    println!("Checking for files removed from input but not in output");
    output_sync(tx, dir.out.to_path_buf(), dir, &mut out_count);
    println!("  Found: {}", out_count);

    println!("Checking for files added to input but not to output");
    input_sync(tx, dir.inn.to_path_buf(), dir, &mut inn_count);
    println!("  Found: {}", inn_count);
}

/// Entrypoint for deleting all files in output folder that are not in input folder.
fn output_sync(tx: &Sender<Actions>, out_path: PathBuf, dir: &Dir, oc: &mut u64) {
    dbg!(&out_path);
    if out_path.is_dir() {
        let entries: Vec<Result<DirEntry, io::Error>> = read_dir(out_path.as_path()).unwrap().into_iter().collect();
        if entries.len() != 0 {
            for entry in entries {
                let path = entry.unwrap().path();
                recursive_output_sync(tx, path, dir, oc);
            } 
        }
    } else {
        if let Some(path) = missing_input(out_path.clone(), dir) {
            tx.send(Actions::Remove(path)).unwrap();
            *oc += 1;
        }
    }
}

/// Recursivley looks for files in output but not in inputs and marks them for deletion.
fn recursive_output_sync(tx: &Sender<Actions>, out_path: PathBuf, dir: &Dir, oc: &mut u64) {
    dbg!(&out_path);
    if out_path.is_dir() {
        let entries: Vec<Result<DirEntry, io::Error>> = read_dir(out_path.as_path()).unwrap().into_iter().collect();
        if entries.len() != 0 {
            for entry in entries {
                let path = entry.unwrap().path();
                recursive_output_sync(tx, path, dir, oc);
            } 
        } else {
            tx.send(Actions::Remove(output_to_input(&out_path, dir))).unwrap();
        }
    } else {
        if let Some(path) = missing_input(out_path.clone(), dir) {
            tx.send(Actions::Remove(path)).unwrap();
            *oc += 1;
        }
    }
}

/// Returns some if output file does not have equivalent input file.
pub fn missing_input(out_path: PathBuf, dir: &Dir) -> Option<PathBuf> {
    match find_input(&out_path, dir) {
        Some(_) => None,
        None => Some(output_to_input(&out_path, dir)),
    }
}

/// Reqursively checks if files in input folder are missing in output and marks them for copy/compression.
fn input_sync(tx: &Sender<Actions>, inn_path: PathBuf, dir: &Dir, ic: &mut u64) {
    if inn_path.is_dir() {
        for entry in read_dir(inn_path.as_path()).unwrap() {
            let path = entry.unwrap().path();
            input_sync(tx, path, dir, ic);
        }
    } else {
        if !input_to_output(&inn_path, dir).is_file() {
            dbg!(&inn_path);
            tx.send(Actions::Copy(inn_path)).unwrap();
            *ic += 1;
        }
    }
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
