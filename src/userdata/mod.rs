use std::collections::HashMap;

mod loader;
pub use self::loader::{load_config, load_keybinds, load_modlist};

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
impl PrefsData {
    pub fn new() -> PrefsData {
        PrefsData {
            volume_game: 0.0,
            volume_music: 0.0,
            volume_ambient: 0.0,
            ui_scale: 0.0,
            hats_only_on_maps: false,
            adaptive_training_enabled: false,
            preferred_names: String::new(),
            resource_readout_categorized: false,
            run_in_background: false,
            custom_cursor_enabled: false,
            edge_screen_scroll: false,
            temperature_mode: String::new(),
            autosave_interval_days: 0,
            show_realtime_clock: false,
            max_number_player_homes: 0,
            plant_wind_sway: false,
            pause_on_load: false,
            pause_on_urgent_letter: false,
            animal_name_mode: String::new(),
            dev_mode: false,
            lang_folder_name: String::new(),
            log_verbose: false,
            pause_on_error: false,
            reset_mods_config_on_crash: false,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct KeyBind {
    key: String,
    value: HashMap<String, String>,
}