use std::thread::sleep;
use std::time::Duration;
use crate::config::get_config;
#[cfg(target_os = "unix")]
use fork::{daemon, Fork};



#[cfg(target_os = "unix")]
pub fn launch_isaac_and_quit() {
    let bin_path = get_config().lock().unwrap().get_binary_path().clone();
    if let Ok(Fork::Child) = daemon(false, false) {
        std::process::Command::new(&bin_path)
            .spawn()
            .expect("failed to start external executable");
    };
    std::process::exit(0);
}


#[cfg(target_os = "windows")]
pub fn launch_isaac_and_quit() {
    let bin_path = get_config().lock().unwrap().get_binary_path().clone();
    CreateProcessW::Command::new(&bin_path)
        .inherit_handles(false)
        .spawn()
        .expect("Error spawning process");
    sleep(Duration::from_secs(3));
    std::process::exit(0);
}
