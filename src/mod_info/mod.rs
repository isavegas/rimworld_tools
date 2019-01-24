use semver::Version;

use std::collections::HashMap;

mod loader;
pub use self::loader::load_mods;

#[derive(Clone,Debug)]
pub struct ModMetaData {
    pub author: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub supported_versions: Vec<Version>,
    pub target_version: Option<Version>, // for old versions
    pub description: Option<String>,
}
impl ModMetaData {
    pub fn new() -> ModMetaData {
        ModMetaData {
            author: None,
            name: None,
            url: None,
            supported_versions: vec!(),
            target_version: None,
            description: None,
        }
    }
}

#[derive(Clone,Debug)]
pub struct Mod {
    pub meta_data: ModMetaData,
    pub version: Option<Version>,
    pub id: Option<i32>,
}
impl Mod {
    pub fn new() -> Mod {
        Mod {
            meta_data: ModMetaData::new(),
            version: None,
            id: None,
        }
    }
}

// Low level representation for .rwc files.
/** 
    TODO: Higher level API for retrieving known data?
        Use Class name and validate def?
*/
#[derive(Debug)]
#[allow(dead_code)]
struct Scenario {
    game_version: String,
    mod_ids: Vec<String>,
    mod_names: Vec<String>,
    name: String,
    description: String,
    published_file_id: String,
    player_faction: String,
    // (li Class, {tag -> tag text})
    parts: (String, HashMap<String, String>),
}
