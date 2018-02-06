#![cfg(target_os = "windows")]
#![allow(dead_code)]

use std::path::PathBuf;
use std::env;

const APPID: & 'static str = "294100";
const LIBRARY_PATH: & 'static str = "C:\\Program Files (x86)\\Steam";
const WORKSHOP_PATH: & 'static str = "steamapps\\workshop\\content";
const GAME_PATH: & 'static str = "steamapps\\common\\RimWorld";

const USERDATA_PATH: & 'static str = "AppData\\LocalLow\\Ludeon Studios\\RimWorld by Ludeon Studios";

pub struct Platform {}

impl Platform {
    pub fn platform_name() -> String {
        String::from("Windows")
    }
    pub fn install_location() -> PathBuf {
        let mut path_buf = PathBuf::from(LIBRARY_PATH);
        path_buf.push(GAME_PATH);
        path_buf
    }
    pub fn workshop_location() -> PathBuf {
        let mut path_buf = PathBuf::from(LIBRARY_PATH);
        path_buf.push(WORKSHOP_PATH);
        path_buf.push(APPID);
        path_buf
    }
    pub fn userdata_location() -> PathBuf {
        if let Ok(path) = env::var("USERPROFILE") {
            let mut path_buf = PathBuf::from(path);
            path_buf.push(USERDATA_PATH);
            path_buf
        } else {
            panic!("Unable to find user directory.");
        }
    }
}