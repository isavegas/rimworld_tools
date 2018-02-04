extern crate quick_xml;

use std::fs::{self};
use std::path::{Path, PathBuf};

use quick_xml::reader::Reader;
use quick_xml::events::Event;

mod semver;
use semver::Version;

// On 1.0, this might change.
const APPID: & 'static str = "294100";
const LIBRARY_PATH: & 'static str = "F:\\SteamLibrary";
const WORKSHOP_PATH: & 'static str = "steamapps\\workshop\\content";
const GAME_PATH: & 'static str = "steamapps\\common\\RimWorld";


#[derive(Debug)]
struct ModMetaData {
    author: Option<String>,
    name: Option<String>,
    url: Option<String>,
    target_version: Option<Version>,
    description: Option<String>,
}
impl ModMetaData {
    fn new() -> ModMetaData {
        ModMetaData {
            author: None,
            name: None,
            url: None,
            target_version: None,
            description: None,
        }
    }
}
#[derive(Debug)]
struct Mod {
    meta_data: ModMetaData,
    version: Option<Version>,
    id: Option<i32>,
}
impl Mod {
    fn new() -> Mod {
        Mod {
            meta_data: ModMetaData::new(),
            version: None,
            id: None,
        }
    }
}

fn load_mods(path: &Path) -> Vec<Mod> {
    let mut mods = Vec::new();
    for path_dir in fs::read_dir(path).expect("Unable to retrieve mods directory list") {
        let entry = path_dir.expect("unable to get entry");
        let mut about_xml_path_buf = PathBuf::from(entry.path().clone());
        about_xml_path_buf.push("About\\about.xml");
        if let Ok(mut reader) = Reader::from_file(about_xml_path_buf.as_path()) {
            reader.trim_text(true);

            let mut mod_info = Mod::new();
            // Get workshop item id
            if let Ok(file_name) = entry.file_name().into_string() {
                if let Ok(id) = file_name.parse::<i32>() {
                    mod_info.id = Some(id);
                }
            }
            let mut buf = Vec::new();
            let mut name = String::new();
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => {
                        if let Ok(text) = String::from_utf8(e.name().to_vec()) {
                            name = text;
                        } else {
                            name = String::new();
                        }
                    },
                    Ok(Event::Text(e)) => {
                        if let Ok(text) = e.unescape_and_decode(&reader) {
                            // No point in messing with it if we have no data.
                            if text.len() != 0 {
                                match name.as_str() {
                                    "name" => mod_info.meta_data.name = Some(text),
                                    "author" => mod_info.meta_data.author = Some(text),
                                    "description" => mod_info.meta_data.description = Some(text),
                                    "url" => mod_info.meta_data.url = Some(text),
                                    "targetVersion" => {
                                        let version_result = Version::from_str(text.as_str());
                                        if let Ok(version) = version_result {
                                            mod_info.meta_data.target_version = Some(version);
                                        } else if let Err(err) = version_result {
                                            println!("Error parsing targetVersion: {}", err);
                                        }
                                    },
                                    _ => ()
                                }
                            }
                        } else {
                            println!("Error decoding text")
                        }
                    },
                    Ok(Event::Eof) => break,
                    _ => (),
                }
            }
            mods.push(mod_info);
        }
    }
    mods
}

fn main() {
    let mut workshop_path_buf = PathBuf::from(LIBRARY_PATH);
    workshop_path_buf.push(WORKSHOP_PATH);
    workshop_path_buf.push(APPID);
    load_mods(workshop_path_buf.as_path()).iter().for_each(|mod_info| println!("{:?}", mod_info));

    let mut mods_path_buf = PathBuf::from(LIBRARY_PATH);
    mods_path_buf.push(GAME_PATH);
    mods_path_buf.push("Mods");
    load_mods(mods_path_buf.as_path()).iter().for_each(|mod_info| println!("{:?}", mod_info));
}
