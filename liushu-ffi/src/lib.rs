uniffi::include_scaffolding!("lib");

use std::path::PathBuf;

use liushu_core::dirs::MyProjectDirs;
pub use liushu_core::engine::candidates::{Candidate, CandidateSource};
pub use liushu_core::engine::{Engine, InputMethodEngine};
pub use liushu_core::error::LiushuError;

trait FfiConstructor {
    fn new() -> Self;
}

impl FfiConstructor for Engine {
    fn new() -> Self {
        // TODO: remove hardcode
        let my_dir = MyProjectDirs {
            data_dir: PathBuf::from("/data/data/com.elliot00.liushu/files"),
            target_dir: PathBuf::from("/data/data/com.elliot00.liushu/files"),
            config_dir: PathBuf::from("/data/data/com.elliot00.liushu/files"),
        };

        Engine::init(&my_dir).unwrap()
    }
}
