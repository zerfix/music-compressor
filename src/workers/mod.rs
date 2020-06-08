pub mod compressor;
pub mod input_listener;

pub enum Actions {
    Copy(String),
    Remove(String),
    Rename(String, String), // from, to
}
