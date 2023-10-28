use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use serde::{Deserialize, Serialize};
use crate::get_main_window;
use crate::options::setup_watcher;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct LauncherConfig {
    binary_path: PathBuf,
    options_path: PathBuf,
}

impl LauncherConfig {
    pub fn get_binary_path(&self) -> &PathBuf {
        &self.binary_path
    }
    pub fn get_options_path(&self) -> &PathBuf {
        &self.options_path
    }

    pub fn set_binary_path(&mut self, path: PathBuf) {
        self.binary_path = path;
        self.save_config_and_notify()
    }
    pub fn set_options_path(&mut self, path: PathBuf) {
        self.options_path = path;
        self.save_config_and_notify()
    }

    fn save_config_and_notify(&self) {
        confy::store("tboi_launcher", None, self)
            .expect("Error updating config");
        let win = get_main_window().lock().unwrap().clone().unwrap();
        win.emit("config_update", self).unwrap();
        drop(win);
        setup_watcher(self.get_options_path()).ok(); // dont care about error here
    }
}

static CFG: OnceLock<Mutex<LauncherConfig>> = OnceLock::new();

pub fn get_config() -> &'static Mutex<LauncherConfig> {
    CFG.get_or_init(|| Mutex::new(
        confy::load("tboi_launcher", None).unwrap()
    ))
}
