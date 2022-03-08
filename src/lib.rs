pub mod accul;

use std::string::String;
use std::{env, path};

pub fn get_program_name(s: &str) -> String {
    return String::from(
        env::current_exe()
            .unwrap_or(path::PathBuf::from(s))
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
    );
}
