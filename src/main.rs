//-////////////////////////////////////////////////////////////////////////////
mod actions {
    pub mod file_system;
    pub mod opus;
    pub mod sync;
}
mod constants;
mod converters {
    pub mod path;
}
mod services {
    pub mod compressor;
    pub mod input_listener;
}
mod types {
    pub mod com;
    pub mod config;
}
//-////////////////////////////////////////////////////////////////////////////

#[macro_use]
extern crate lazy_static;

use std::sync::mpsc::channel;
use std::thread;

use actions::file_system::init_directories;
use actions::sync::init_sync;
use services::compressor::compress;
use services::input_listener::listen;
use types::config::Config;

//-////////////////////////////////////////////////////////////////////////////
//  static
//-////////////////////////////////////////////////////////////////////////////
lazy_static!{
    static ref CONF: Config = Config::from_env();
}

//-////////////////////////////////////////////////////////////////////////////
//  main
//-////////////////////////////////////////////////////////////////////////////
fn main() {
    init_directories(&CONF.dir).unwrap();

    // initial sync
    let (tx, rx) = channel();
    println!("--- T0 Comparing -------------");
    init_sync(&tx, &CONF.dir);

    // listen thread
    let _listener = {
        thread::Builder::new()
            .name("compressor".to_string())
            .spawn(move || listen(tx))
            .unwrap()
    };
    println!("--- T1 Listening -------------");

    // compress
    println!("--- T0 Compressing -----------");
    compress(rx);
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
