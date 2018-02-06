extern crate quick_xml;

use std::path::Path;

use self::quick_xml::reader::Reader;
use self::quick_xml::events::Event;

use ::userdata::{KeyBind, ModsConfigData, PrefsData};
use ::userdata::Save;


pub fn load_modlist(path: &Path) -> Result<ModsConfigData, String> {
    if let Ok(mut reader) = Reader::from_file(path) {
        reader.trim_text(true);
        let mut build_number: i32 = 0;
        let mut active_mods: Vec<String> = Vec::new();
        let mut name = String::new();
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if let Ok(text) = String::from_utf8(e.name().to_vec()) {
                        name = text;
                    } else {
                        return Err(String::from("Malformed tag name!"));
                    }
                },
                Ok(Event::Text(e)) => {
                    if let Ok(text) = e.unescape_and_decode(&reader) {
                        match name.as_str() {
                            "buildNumber" => {
                                let parse_result = text.parse();
                                if let Ok(build) = parse_result {
                                    build_number = build;
                                } else {
                                    return Err(format!("Unable to parse build number: `{}` {:?}", text, parse_result.unwrap_err()));
                                }
                            },
                            "li" => {
                                active_mods.push(text);
                            },
                            _ => (),
                        }
                    }
                },
                Ok(Event::Eof) => break,
                _ => (),
            }
        }

        Ok(ModsConfigData {
            build_number: build_number,
            active_mods: active_mods
        })
    } else {
        Err(String::from("Unable to load modlist"))
    }
}

#[allow(dead_code)]
pub fn load_config() -> Result<PrefsData, String> {
    unimplemented!()
}

#[allow(dead_code)]
pub fn load_mods_config() -> Result<ModsConfigData, String> {
    unimplemented!()
}

#[allow(dead_code)]
pub fn load_keybinds() -> Result<Vec<KeyBind>, String> {
    unimplemented!()
}

#[allow(dead_code)]
pub fn load_save() -> Result<Save, String> {
    unimplemented!();
}