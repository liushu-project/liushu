use directories::ProjectDirs;

pub fn get_proj_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "elliot00", "liushu")
        .expect("no valid home directory path could be retrieved from the operating system")
}
