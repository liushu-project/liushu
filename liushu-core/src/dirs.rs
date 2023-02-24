use std::path::PathBuf;

use directories::BaseDirs;
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct MyProjectDirs {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub target_dir: PathBuf,
}

pub static PROJECT_DIRS: Lazy<MyProjectDirs> = Lazy::new(|| {
    let base_dirs = BaseDirs::new().expect(
        "there is no valid home directory path could be retrieved from the operating system",
    );
    let data_dir = base_dirs.data_dir().join("liushu");

    MyProjectDirs {
        config_dir: base_dirs.config_dir().join("liushu"),
        target_dir: data_dir.join("target"),
        data_dir,
    }
});
