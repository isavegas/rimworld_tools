use std::collections::HashMap;

mod loader;
pub use self::loader::{load_config, load_mods_config, load_keybinds, load_modlist};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Save {
    
}
#[allow(dead_code)]
impl Save {
    fn new() -> Save {
        unimplemented!();
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ModsConfigData {
    build_number: i32,
    active_mods: Vec<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PrefsData {
    volume_game: f32,
    volume_music: f32,
    volume_ambient: f32,
    ui_scale: f32,
    hats_only_on_maps: bool,
    adaptive_training_enabled: bool,
    preferred_names: String,
    resource_readout_categorized: bool,
    run_in_background: bool,
    custom_cursor_enabled: bool,
    edge_screen_scroll: bool,
    temperature_mode: String,
    autosave_interval_days: i32,
    show_realtime_clock: bool,
    max_number_player_homes: i32,
    plant_wind_sway: bool,
    pause_on_load: bool,
    pause_on_urgent_letter: bool,
    animal_name_mode: String,
    dev_mode: bool,
    lang_folder_name: String,
    log_verbose: bool,
    pause_on_error: bool,
    reset_mods_config_on_crash: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct KeyBind {
    key: String,
    value: HashMap<String, String>,
}