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

use std::sync::mpsc::channel;
use std::thread;

use actions::file_system::init_directories;
use actions::sync::init_sync;
use services::compressor::compress;
use services::input_listener::listen;
use types::config::Config;

//-////////////////////////////////////////////////////////////////////////////
//  main
//-////////////////////////////////////////////////////////////////////////////
fn main() {
    let config = Config::from_env();
    init_directories(&config.dir).unwrap();

    // initial sync
    let (tx, rx) = channel();
    println!("--- T0 Comparing -------------");
    init_sync(&tx, &config.dir);

    // listen thread
    let _listener = {
        let config = config.clone();
        thread::Builder::new()
            .name("compressor".to_string())
            .spawn(move || listen(config, tx))
            .unwrap()
    };
    println!("--- T1 Listening -------------");

    // compress
    println!("--- T0 Compressing -----------");
    compress(rx, config);
}
//-////////////////////////////////////////////////////////////////////////////
//
//-////////////////////////////////////////////////////////////////////////////
