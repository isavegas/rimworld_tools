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
pub fn load_config(path: &Path) -> Result<PrefsData, String> {
    if let Ok(mut reader) = Reader::from_file(path) {
        let mut config = PrefsData::new();
        reader.trim_text(true);
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
                    // Note that RimWorld uses True and False, rather than true and false
                    // We use .to_lowercase() where booleans are expected to fix that.
                    // TODO: implement "True"/"False" -> true/false trait?
                    if let Ok(text) = e.unescape_and_decode(&reader) {
                        match name.as_str() {
                            "volumeGame" => {
                                let parse_result = text.parse();
                                if let Ok(value) = parse_result {
                                    config.volume_game = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "volumeMusic" => {
                                let parse_result = text.parse();
                                if let Ok(value) = parse_result {
                                    config.volume_music = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "volumeAmbient" => {
                                let parse_result = text.parse();
                                if let Ok(value) = parse_result {
                                    config.volume_ambient = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "uiScale" => {
                                let parse_result = text.parse();
                                if let Ok(value) = parse_result {
                                    config.ui_scale = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "hatsOnlyOnMap" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.hats_only_on_maps = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "adaptiveTrainingEnabled" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.adaptive_training_enabled = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "preferredNames" => {
                                // In Prefs.xml, this is usually <preferredNames />,
                                // and won't even trigger this, keeping default ""
                                config.preferred_names = text;
                            },
                            "resourceReadoutCategorized" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.resource_readout_categorized = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "runInBackground" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.run_in_background = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "customCursorEnabled" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.custom_cursor_enabled = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "edgeScreenScroll" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.custom_cursor_enabled = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "temperatureMode" => {
                                config.temperature_mode = text;
                            },
                            "autosaveIntervalDays" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.autosave_interval_days = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "showRealtimeClock" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.show_realtime_clock = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "maxNumberOfPlayerHomes" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.max_number_player_homes = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "plantWindSway" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.plant_wind_sway = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "pauseOnLoad" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.pause_on_load = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "pauseOnUrgentLetter" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.pause_on_urgent_letter = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "animalNameMode" => {
                                config.animal_name_mode = text
                            },
                            "devMode" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.dev_mode = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "langFolderName" => {
                                config.lang_folder_name = text;
                            },
                            "logVerbose" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.log_verbose = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "pauseOnError" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.pause_on_error = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "resetModsConfigOnCrash" => {
                                let parse_result = text.to_lowercase().parse();
                                if let Ok(value) = parse_result {
                                    config.reset_mods_config_on_crash = value;
                                } else {
                                    return Err(format!("Unable to parse {}: `{}` {:?}", name, text, parse_result.unwrap_err()));
                                }
                            },
                            "PrefsData" => (),
                            _ => (),
                        }
                    }
                },
                Ok(Event::Eof) => break,
                _ => (),
            }
        }

        Ok(config)
    } else {
        Err(String::from("Unable to load modlist"))
    }
}

#[allow(dead_code)]
pub fn load_keybinds() -> Result<Vec<KeyBind>, String> {
    unimplemented!()
}

#[allow(dead_code)]
pub fn load_save() -> Result<Save, String> {
    unimplemented!();
}