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
        let my_dir = MyProjectDirs {
            data_dir: PathBuf::from("./Data"),
            target_dir: PathBuf::from("./Target"),
            config_dir: PathBuf::from("./Config"),
        };

        Engine::init(&my_dir).unwrap()
    }
}
