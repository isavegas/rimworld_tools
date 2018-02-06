
mod mod_info;
mod platform;
mod semver;
mod userdata;


use mod_info::load_mods;
use userdata::load_modlist;
use platform::platform::Platform;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("mods")) {
        let mods_result = load_mods(Platform::workshop_location().as_path());
        if let Ok(mods) = mods_result {
            mods.iter().for_each(|mod_info| println!("{:?}", mod_info))
        } else {
            println!("Error loading mods: {}", mods_result.unwrap_err());
        }
    }
    if args.contains(&String::from("mods")) {
        let mut install_location = Platform::install_location();
        install_location.push("Mods");
        let mods_result = load_mods(install_location.as_path());
        if let Ok(mods) = mods_result {
            mods.iter().for_each(|mod_info| println!("{:?}", mod_info))
        } else {
            println!("Error loading mods: {}", mods_result.unwrap_err());
        }
    }
    if args.contains(&String::from("modlist")) {
        let mut userdata_location = Platform::userdata_location();
        userdata_location.push("Config");
        userdata_location.push("ModsConfig.xml");
        let modslist_result = load_modlist(userdata_location.as_path());
        if let Ok(modlist) = modslist_result {
            println!("Active mods: {:?}", modlist);
        } else {
            println!("Error loading modlist: {}", modslist_result.unwrap_err())
        }
    }

    // May be unstable.
    #[cfg(debug_assertions)]
    {
        println!("Install: {:?}", Platform::install_location().as_path());
        println!("Workshop: {:?}", Platform::workshop_location().as_path());
        println!("Userdata: {:?}", Platform::userdata_location().as_path());
    }
}
