extern crate quick_xml;

mod mod_info;
mod mod_loader;
mod platform;
mod semver;
mod userdata;
mod userdata_loader;


use mod_loader::load_mods;
use platform::platform::Platform;


fn main() {

    let mods_result = load_mods(Platform::workshop_location().as_path());
    if let Ok(mods) = mods_result {
        mods.iter().for_each(|mod_info| println!("{:?}", mod_info))
    } else {
        println!("Error loading mods: {}", mods_result.unwrap_err());
    }

    let mut userdata_location = Platform::install_location();
    userdata_location.push("Mods");
    let mods_result = load_mods(userdata_location.as_path());
    if let Ok(mods) = mods_result {
        mods.iter().for_each(|mod_info| println!("{:?}", mod_info))
    } else {
        println!("Error loading mods: {}", mods_result.unwrap_err());
    }

    // May be unstable.
    #[cfg(debug_assertions)]
    {
        println!("Install: {:?}", Platform::install_location().as_path());
        println!("Workshop: {:?}", Platform::workshop_location().as_path());
        println!("Userdata: {:?}", Platform::userdata_location().as_path());
    }
}
