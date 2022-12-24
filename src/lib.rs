use std::{
    path::Path,
    fs::File,
    fs::create_dir_all,
    env::args
};

pub fn get_folder() -> Option<String> {
    args().nth(1)
}