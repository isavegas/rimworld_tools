#![cfg(target_os = "macos")]
#![allow(dead_code)]

use std::path::PathBuf;

pub struct Platform {}

impl Platform {
    pub fn platform_name() -> String {
        String::from("MacOS")
    }
    pub fn install_location() -> PathBuf {
        let path_buf = PathBuf::new();
        path_buf
    }
    pub fn userdata_location() -> PathBuf {
        let path_buf = PathBuf::new();
        path_buf
    }
}