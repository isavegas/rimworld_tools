#![cfg(target_os = "macos")]
#![allow(dead_code)]

use std::path::PathBuf;
use std::env;

const APPID: & 'static str = "294100";
const LIBRARY_PATH: & 'static str = "";
const WORKSHOP_PATH: & 'static str = "steamapps\\workshop\\content";
const GAME_PATH: & 'static str = "";

const USERDATA_PATH: & 'static str = "";

pub struct Platform {}

impl Platform {
    pub fn platform_name() -> String {
        String::from("MacOS")
    }
    pub fn install_location() -> PathBuf {
        unimplemented!()
    }
    pub fn workshop_location() -> PathBuf {
        unimplemented!()
    }
    pub fn userdata_location() -> PathBuf {
        unimplemented!()
    }
}