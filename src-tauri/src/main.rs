// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::sync::{Mutex, OnceLock};
use tauri::{Manager, Window};
use crate::config::{get_config, LauncherConfig};
use crate::options::setup_watcher;
use crate::picker::{IsaacFile, set_file_path};
use crate::program::launch_isaac_and_quit;

mod picker;
mod config;
mod options;
mod program;


#[tauri::command]
fn load_config() -> LauncherConfig {
    let conf = get_config().lock().unwrap();
    let front_conf = conf.clone();
    front_conf
}

#[tauri::command]
fn choose_binary() {
    set_file_path(IsaacFile::Binary)
}


#[tauri::command]
fn choose_options() {
    set_file_path(IsaacFile::Options)
}

#[tauri::command]
fn toggle_debug() -> String {
    match options::toggle_debug() {
        Ok(_) => "".to_string(),
        Err(e) => e.to_string()
    }
}

#[tauri::command]
fn is_in_debug() -> Option<bool> {
    options::is_debug().ok()
}

#[tauri::command]
fn start_isaac() {
    launch_isaac_and_quit()
}

static WIN: OnceLock<Mutex<Option<Window>>> = OnceLock::new();

fn get_main_window() -> &'static Mutex<Option<Window>> {
    WIN.get_or_init(|| Mutex::new(
        None
    ))
}



fn main() {
    let opt_path = get_config().lock().unwrap().get_options_path().clone();
    setup_watcher(&opt_path).ok();
    drop(opt_path);

    tauri::Builder::default()
        .setup(|app| {
            let mut win = get_main_window().lock()?;
            *win = app.get_window("main");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_config, choose_binary, choose_options, toggle_debug, is_in_debug, start_isaac
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
