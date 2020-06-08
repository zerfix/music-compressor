// std
use std::ffi::OsStr;
use std::fs::{copy, create_dir_all, remove_file};
use std::path::Path;
// local
use super::super::formats::opus::compress;
use super::super::formats::{LOSSLESS, LOSSY, OTHER};
use super::folders::delete_dir;


//-////////////////////////////////////////////////////////////////////////////
// Files
//-////////////////////////////////////////////////////////////////////////////
pub fn file_copy(innpath: String, outpath: String) {
    let path = Path::new(&innpath);

    if path.is_dir() {
        let path = Path::new(&outpath);
        if !path.exists() {
            create_dir_all(&path).unwrap();
        }
        println!("Copied dir: {}", innpath);
        return;
    }

    {
        let path = Path::new(&outpath).parent().unwrap();
        if !path.exists() {
            create_dir_all(path).unwrap();
            println!("Created dir: {}", path.to_str().unwrap());
        }
    }

    let ext = match path.extension() {
        Some(s) => s,
        None => OsStr::new(""),
    };

    for e in LOSSY.iter().chain(OTHER.iter()) {
        if ext == *e {
            copy(path, outpath).unwrap();
            println!("Copied: {}", innpath);
            return;
        }
    }

    for e in LOSSLESS.iter() {
        if ext == *e {
            compress(innpath.clone(), outpath).unwrap();
            println!("Compressed: {}", innpath);
            return;
        }
    }

    println!("File not toutched: {:?}", innpath);
}

pub fn file_remove(path: String) {
    let targetpath = Path::new(&path);
    if targetpath.exists() {
        if !targetpath.is_dir() {
            match remove_file(targetpath) {
                Ok(_) => (),
                Err(err) => println!("File removal error: {:?}", err),
            }
        } else {
            match delete_dir(path.clone()) {
                Ok(_) => (),
                Err(err) => println!("Dir removal error: {:?}", err),
            }
        }
    }
    println!("Removed: {}", path);
}

pub fn file_rename(old: String, new_inn: String, new_out: String) {
    file_remove(old);
    file_copy(new_inn, new_out);
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
