use std::path::PathBuf;
use std::env::consts::ARCH;
use sysinfo::{System, SystemExt};

pub fn get_app_dir() -> PathBuf {
    let path = platform_dirs::AppDirs::new(Some("<your_app_name>"), true).unwrap(); // <-- Edit the "<your_app_name>" to your app name
    let mut data_dir = path.data_dir;
    // check if in dev mode
    if cfg!(debug_assertions) {
        data_dir.push("dev");
    }
    data_dir
}

pub fn get_os_info() -> (String, String) {
    let mut sys = System::new_all();
    sys.refresh_all();
    let os = sys.name().unwrap().to_lowercase();
    let arch = ARCH.to_lowercase();
    
    (os, arch)
}