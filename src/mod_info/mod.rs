use semver::Version;

mod loader;
pub use self::loader::load_mods;

#[derive(Debug)]
pub struct ModMetaData {
    pub author: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub target_version: Option<Version>,
    pub description: Option<String>,
}
impl ModMetaData {
    pub fn new() -> ModMetaData {
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