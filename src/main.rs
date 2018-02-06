
mod mod_info;
mod platform;
mod semver;
mod userdata;


use mod_info::{ load_mods, Mod };
use userdata::{ load_config, load_modlist };
use platform::platform::Platform;
use semver::Version;

use std::env;

fn main() {
    let current_rimworld_version = Version::from_str("0.18.0").unwrap();
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("mods")) {
        let mods_result = load_mods(Platform::workshop_location().as_path());
        if let Ok(mods) = mods_result {
            mods.iter().for_each(|m| println!("{:?}", m));
        } else {
            println!("Error loading mods: {}", mods_result.unwrap_err());
        }
    }
    if args.contains(&String::from("mods")) {
        let mut install_location = Platform::install_location();
        install_location.push("Mods");
        let mods_result = load_mods(install_location.as_path());
        if let Ok(mods) = mods_result {
            mods.iter().for_each(|mod_info| println!("{:#?}", mod_info));
        } else {
            println!("Error loading mods: {}", mods_result.unwrap_err());
        }
    }


    if args.contains(&String::from("outdated")) {
        let mods_result = load_mods(Platform::workshop_location().as_path());
        let mut outdated: Vec<Mod> = Vec::new();
        let blank_version = Version::new(0,0,0);
        if let Ok(mods) = mods_result {
            // You need .as_ref() because otherwise Option<T>::unwrap() will consume the
            // option and move the contained value into the current context.
            outdated = mods.iter().filter(|m| m.meta_data.target_version.as_ref().unwrap_or(&blank_version) < &current_rimworld_version).map(|m| m.clone()).collect::<Vec<Mod>>();
        } else {
            println!("Error loading mods: {}", mods_result.unwrap_err());
        }
        outdated.iter().for_each(|m| println!("Outdated: ({}) {}", m.meta_data.target_version.as_ref().unwrap_or(&blank_version).to_string(), m.meta_data.name.as_ref().unwrap_or(&String::from("[Invalid Name]"))))
    }
    if args.contains(&String::from("outdated")) {
        let mut install_location = Platform::install_location();
        install_location.push("Mods");
        let mods_result = load_mods(install_location.as_path());
        let mut outdated: Vec<Mod> = Vec::new();
        let blank_version = Version::new(0,0,0);
        if let Ok(mods) = mods_result {
            // You need .as_ref() because otherwise Option<T>::unwrap() will consume the
            // option and move the contained value into the current context.
            outdated = mods.iter().filter(|m| m.meta_data.target_version.as_ref().unwrap_or(&blank_version) < &current_rimworld_version).map(|m| m.clone()).collect::<Vec<Mod>>();
        } else {
            println!("Error loading mods: {}", mods_result.unwrap_err());
        }
        outdated.iter().for_each(|m| println!("Outdated: ({}) {}", m.meta_data.target_version.as_ref().unwrap_or(&blank_version).to_string(), m.meta_data.name.as_ref().unwrap_or(&String::from("[Invalid Name]"))))
    }
    if args.contains(&String::from("modlist")) {
        let mut userdata_location = Platform::userdata_location();
        userdata_location.push("Config");
        userdata_location.push("ModsConfig.xml");
        let modslist_result = load_modlist(userdata_location.as_path());
        if let Ok(modlist) = modslist_result {
            println!("Active mods: {:#?}", modlist);
        } else {
            println!("Error loading modlist: {}", modslist_result.unwrap_err())
        }
    }
    if args.contains(&String::from("config")) {
        let mut userdata_location = Platform::userdata_location();
        userdata_location.push("Config");
        userdata_location.push("Prefs.xml");
        let config_result = load_config(userdata_location.as_path());
        if let Ok(config) = config_result {
            println!("Current configuration: {:#?}", config);
        } else {
            println!("Error loading config: {}", config_result.unwrap_err())
        }
    }

    // May be unstable.
    #[cfg(debug_assertions)]
    {
        // Path is a pain to convert to &str. Easier to let std::fmt handle it.
        println!("Install: {:?}", Platform::install_location().as_path());
        println!("Workshop: {:?}", Platform::workshop_location().as_path());
        println!("Userdata: {:?}", Platform::userdata_location().as_path());
    }
}
