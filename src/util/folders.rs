// std
use std::ffi::OsStr;
use std::fs::DirBuilder;
use std::fs::remove_dir;
use std::io;
use std::path::Path;
// local
use super::super::formats::LOSSLESS;
use super::config::Config;


//-////////////////////////////////////////////////////////////////////////////
// Paths
//-////////////////////////////////////////////////////////////////////////////
pub fn inputpath_to_outputpath(path: String, input_dir: &str, output_dir: &str) -> String {
    let path = Path::new(&path);
    let is_lossless: bool = {
        let mut is_lossless = false;
        if let Some(ext) = path.extension() {
            for n in LOSSLESS.iter() {
                if ext == OsStr::new(n) {
                    is_lossless = true;
                    break;
                }
            }
        }
        is_lossless
    };
    if is_lossless {
        Path::new(output_dir)
            .join(path.strip_prefix(input_dir).unwrap())
            .with_extension(OsStr::new("opus"))
            .to_str()
            .unwrap()
            .to_string()
    } else {
        Path::new(output_dir)
            .join(path.strip_prefix(input_dir).unwrap())
            .to_str()
            .unwrap()
            .to_string()
    }
}
//-////////////////////////////////////////////////////////////////////////////
// Directories
//-////////////////////////////////////////////////////////////////////////////
pub fn create_input_dir(config: &Config) -> io::Result<()> {
    create_dir(config.input_dir.clone())
}

pub fn create_output_dir(config: &Config) -> io::Result<()> {
    create_dir(config.output_dir.clone())
}

fn create_dir(path: String) -> io::Result<()> {
    if !Path::new(&path).exists() {
        DirBuilder::new().create(path)?;
    }
    Ok(())
}

pub fn delete_dir(path: String) -> io::Result<()> {
    remove_dir(path)?;
    Ok(())
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
