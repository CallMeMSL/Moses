use tauri::api::dialog::FileDialogBuilder;
use crate::config::get_config;

pub enum IsaacFile {
    Binary,
    Options
}

pub fn set_file_path(file: IsaacFile) {
    let chosen_file = FileDialogBuilder::default();
    match file {
        IsaacFile::Binary => {chosen_file.add_filter("Binary", &["exe"])}
        IsaacFile::Options => {chosen_file.add_filter("Isaac Options File", &["ini"])}
    }.pick_file(move |fp| {
        if let Some(file_path) = fp {
            let mut cfg = get_config().lock().expect("Can't acquire config lock");
            match file {
                IsaacFile::Binary => {cfg.set_binary_path(file_path)}
                IsaacFile::Options => {cfg.set_options_path(file_path)}
            }
        };
    });
}


