mod formats;
mod util;
mod workers;
// std
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::path::Path;
use std::fs::read_dir;
// local
use util::config::Config;
use util::folders::create_input_dir;
use util::folders::create_output_dir;
use util::folders::inputpath_to_outputpath;
use workers::Actions;
use workers::compressor::compressor;
use workers::input_listener::listen;


//-////////////////////////////////////////////////////////////////////////////
// main
//-////////////////////////////////////////////////////////////////////////////
/// this thread does init and file listening
fn main() {
    // init
    let config = Config::from_env();
    let (tx, rx) = channel();

    create_input_dir(&config).unwrap();
    create_output_dir(&config).unwrap();

    println!("--- T0 Comparing -------------");

    // initial sync
    init_sync(Path::new(&config.input_dir), &tx, &config.input_dir[..], &config.output_dir[..]);

    {
        let config = config.clone();
        thread::Builder::new()
            .name("compressor".to_string())
            .spawn(move || listen(config, tx).unwrap())
            .unwrap();
    }
    println!("--- T1 Listening -------------");

    println!("--- T0 Compressing -----------");
    // listen
    compressor(rx, config);
}

//-////////////////////////////////////////////////////////////////////////////
// initial syncronization
//-////////////////////////////////////////////////////////////////////////////
pub fn init_sync(path: &Path, tx: &Sender<Actions>, inn_dir: &str, out_dir: &str) {
    if path.is_dir() {
        for entry in read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            init_sync(path.as_path(), tx, inn_dir, out_dir);
        }
    } else {
        // println!("Input path: {}", path.to_str().unwrap());
        let output = inputpath_to_outputpath(path.to_str().unwrap().to_string(), inn_dir, out_dir);
        // println!("Output path: {}", &output);
        if !Path::new(&output).exists() {
            tx.send(Actions::Copy(path.to_str().unwrap().to_string())).unwrap();
        };
    }
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
