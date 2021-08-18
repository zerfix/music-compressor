use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use dotenv::dotenv;

//-////////////////////////////////////////////////////////////////////////////
//  Programm Config
//-////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct Config {
    pub dir: Dir,
}

#[derive(Clone)]
pub struct Dir {
    pub inn: PathBuf,
    pub out: PathBuf,
}

impl Config {
    pub fn from_env() -> Config {
        dotenv().ok();
        let mut entries: HashMap<_, _> = ["INPUT_DIR", "OUTPUT_DIR"]
            .iter()
            .map(|x| match env::var(x) {
                Ok(k) => (*x, k),
                Err(err) => panic!("ENV variable {} is missing!\n{}", x, err),
            })
            .collect();
        Config {
            dir: Dir {
                inn: PathBuf::from(entries.remove("INPUT_DIR").unwrap()),
                out: PathBuf::from(entries.remove("OUTPUT_DIR").unwrap()),
            }
        }
    }
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
