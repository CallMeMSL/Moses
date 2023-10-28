use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use regex::{Regex};
use crate::config::get_config;
use hotwatch::{Hotwatch, Event, EventKind};
use crate::get_main_window;

pub enum ConfigCheckResult {
    InvalidEntryFound,
    Enabled,
    Disabled,
    NoEntryFound,
}

#[derive(Debug)]
pub struct OptionsError {
    msg: String
}

impl Display for OptionsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OptionsError: {}", self.msg)
    }
}

impl Error for OptionsError {}

static OPTIONS_WATCHER: OnceLock<Mutex<Option<Hotwatch>>> = OnceLock::new();

pub fn get_options_watcher() -> &'static Mutex<Option<Hotwatch>> {
    OPTIONS_WATCHER.get_or_init(|| Mutex::new(
        None
    ))
}

pub fn setup_watcher(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut hotwatch = Hotwatch::new()?;
    hotwatch.watch(path, |event: Event| {
        if let EventKind::Modify(_) = event.kind {
            let win = get_main_window().lock().unwrap().clone().unwrap();
            win.emit("debug_file_update", is_debug().ok()).unwrap();
        }
    })?;
    let mut watcher_shell = get_options_watcher().lock()?;
    *watcher_shell = Some(hotwatch);
    Ok(())
}

static REGEX: OnceLock<Regex> = OnceLock::new();

pub fn check_config_content(content: &String) -> Result<ConfigCheckResult, Box<dyn Error>> {
    let r = REGEX.get_or_init(|| Regex::new(r"EnableDebugConsole=(.)").unwrap());
    let res = r.find(&content);
    match res {
        None => Ok(ConfigCheckResult::NoEntryFound),
        Some(m) => {
            match m.as_str() {
                "EnableDebugConsole=1" => Ok(ConfigCheckResult::Enabled),
                "EnableDebugConsole=0" => Ok(ConfigCheckResult::Disabled),
                _ => Ok(ConfigCheckResult::InvalidEntryFound)
            }
        }
    }
}

pub fn toggle_debug_console_in_string(content: &mut String) -> Result<(), Box<dyn Error>> {
    let match_res = check_config_content(content)?;
    match match_res {
        ConfigCheckResult::Enabled => { *content = content.replace("EnableDebugConsole=1", "EnableDebugConsole=0") }
        ConfigCheckResult::Disabled => { *content = content.replace("EnableDebugConsole=0", "EnableDebugConsole=1") }
        ConfigCheckResult::NoEntryFound => { content.push_str("\nEnableDebugConsole=0\n") }
        ConfigCheckResult::InvalidEntryFound => { return Err(Box::new(OptionsError {
            msg: "Debug entry was found, but the given value is not 0/1".to_string()}));
        }
    };
    Ok(())
}

pub fn modify_options_file(options_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .open(options_path)?;
    let mut file_string = String::new();
    file.read_to_string(&mut file_string)?;
    toggle_debug_console_in_string(&mut file_string)?;
    file.rewind()?;
    file.write(file_string.as_ref())?;
    Ok(())
}

pub fn toggle_debug() -> Result<(), Box<dyn Error>> {
    let config = get_config().lock()?; // do i wanna copy or hold the lock?
    modify_options_file(config.get_options_path())
}

pub fn is_debug() -> Result<bool, Box<dyn Error>> {
    let config = get_config().lock()?; // do i wanna copy or hold the lock?
    let content: String = fs::read_to_string(config.get_options_path())?.parse()?;
    drop(config);
    match check_config_content(&content)? {
        ConfigCheckResult::Enabled => Ok(true),
        ConfigCheckResult::Disabled => Ok(false),
        ConfigCheckResult::InvalidEntryFound => Err(Box::new(OptionsError {msg: "An invalid entry for EnableDebugConsole was found in the file.".to_string()})),
        ConfigCheckResult::NoEntryFound => Err(Box::new(OptionsError {msg: "No entry for debug was found. Click once on swap mode to add one.".to_string()}))
    }

}

