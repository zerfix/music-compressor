use std::fs;
use std::io;
use std::path::PathBuf;

use crate::actions::opus::compress;
use crate::constants::{LOSSLESS, LOSSY, OTHER};
use crate::converters::path::output_to_input;
use crate::types::config::Dir;

//-////////////////////////////////////////////////////////////////////////////
//  File System Actions
//-////////////////////////////////////////////////////////////////////////////
/// Initializes the input and output directories.
pub fn init_directories(dir: &Dir) -> io::Result<()> {
    for dir in [&dir.inn, &dir.out].iter() {
        if !dir.exists() {
            fs::create_dir_all(dir)?;
        }
    }
    Ok(())
}

/// Copy file to output folder.
pub fn copy(inn_path: PathBuf, out_path: PathBuf) -> io::Result<String> {
    if inn_path.is_dir() { 
        return Ok(format!("Is dir: {:?}", inn_path));
    }

    let parent = out_path.parent().unwrap();
    if !parent.exists() { fs::create_dir_all(parent)?; }

    if let Some(extension) = inn_path.extension() {
        if let Some(ext) = extension.to_str() {
            if LOSSLESS.contains(&ext) && inn_path.exists() {
                compress(&inn_path, &out_path)?;
                return Ok(format!("Compressed: {:?}", inn_path))
            }
        
            if (LOSSY.contains(&ext) || OTHER.contains(&ext)) && inn_path.exists() {
                fs::copy(&inn_path, out_path)?;
                return Ok(format!("Copied file: {:?}", inn_path));
            }
        }
    }

    Ok(format!("File not touched: {:?}", inn_path))
}

/// Remove file from output folder.
pub fn remove(path: PathBuf) -> io::Result<String> {
    if path.exists() {
        if path.is_dir() {
            fs::remove_dir(&path)?;
            return Ok(format!("Removed dir: {:?}", path));
        } else {
            fs::remove_file(&path)?;
            return Ok(format!("Removed file: {:?}", path));
        }
    } else {
        return Ok(format!("Remove: File not found! {:?}", path));
    }
}

/// Remove old then copy new file to output folder.
pub fn rename(old_out: PathBuf, new_inn: PathBuf, new_out: PathBuf) -> io::Result<String> {
    remove(old_out.clone())?;
    copy(new_inn.clone(), new_out.clone())?;
    Ok(format!("Moved: {:?}\n To: {:?}", old_out, new_out))
}

//-////////////////////////////////////////////////////////////////////////////
//  Reverse Find
//-////////////////////////////////////////////////////////////////////////////
/// Find lossless input file that the output opus file originated from.
pub fn find_input(out_path: &PathBuf, dir: &Dir) -> Option<PathBuf> {
    let inn_path = output_to_input(out_path, dir);

    if inn_path.exists() {
        return Some(inn_path);
    }

    if let Some(extension) = out_path.extension() {
        if let Some(ext) = extension.to_str() {
            if ext == "opus" {
                for e in LOSSLESS.iter().chain(["opus"].iter()) {
                    let path = inn_path.with_extension(e);
                    if path.exists() {
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
