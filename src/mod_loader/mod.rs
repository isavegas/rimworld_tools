
use std::fs::{self};
use std::path::{Path, PathBuf};

use quick_xml::reader::Reader;
use quick_xml::events::Event;

use ::mod_info::Mod;
use ::semver::Version;


pub fn load_mods(path: &Path) -> Result<Vec<Mod>, String> {
    let mut mods = Vec::new();

    if let Ok(dir_iter) = fs::read_dir(path) {
        for path_dir in dir_iter {
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
        Ok(mods)
    } else {
        Err(format!("Unable to access {:?}", path))
    }
}