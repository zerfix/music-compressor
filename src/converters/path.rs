use std::ffi::OsStr;
use std::path::PathBuf;

use crate::constants::LOSSLESS;
use crate::types::config::Dir;

//-////////////////////////////////////////////////////////////////////////////
//  Path Transforms
//-////////////////////////////////////////////////////////////////////////////
/// Convert input path to its equivalent output path.
/// Also converts extention from lossless to opus.
pub fn input_to_output(inn_path: &PathBuf, dir: &Dir) -> PathBuf {
    let mut out_path = dir.out.join(inn_path.strip_prefix(dir.inn.clone()).unwrap());

    if let Some(extension) = inn_path.extension() {
        if let Some(ext) = extension.to_str() {
            if LOSSLESS.contains(&ext) { 
                out_path = out_path.with_extension(OsStr::new("opus")); 
            }
        }
    }

    out_path
}

/// Convert output path to its equivalent input path.
pub fn output_to_input(out_path: &PathBuf, dir: &Dir) -> PathBuf {
    dir.inn.join(out_path.strip_prefix(dir.out.clone()).unwrap())
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
