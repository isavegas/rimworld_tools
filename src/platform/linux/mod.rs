#![cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
#![allow(dead_code)]

use std::path::PathBuf;

pub struct Platform {}

impl Platform {
    pub fn platform_name() -> String {
        String::from(cfg!(target_os))
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