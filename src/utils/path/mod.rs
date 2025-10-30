
use std::{env, fs, path::PathBuf};

mod path_matchers;



pub fn get_current_path() -> PathBuf {
    env::current_dir().unwrap().join("demo")
}

pub fn get_project_root_path() -> PathBuf {
    get_current_path()
}

pub fn get_split_key() -> &'static str {
    std::path::MAIN_SEPARATOR_STR
}

pub fn get_core_module_names() -> Option<Vec<String>> {
    let core_dir_path = get_project_root_path().join("src").join("core");
    let mut core_module_names = Vec::new();

    for entry in match fs::read_dir(core_dir_path) {
        Ok(entries) => entries,
        Err(_) => return None,
    } {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() {
                    core_module_names.push(path.file_name()?.to_str()?.to_string());
                }
            }
            Err(_) => continue,
        }
    }
    Some(core_module_names)
}

pub fn str_path_is_dir(path: String) -> bool {
    let path = PathBuf::from(&path);
    path.is_dir()
}


#[derive(Debug)]
pub enum PathFlag {
    RootFunction,
    CoreFunction,
    EnvRootFunction,
    EnvCoreFunction,
    NotFound,
}

#[derive(Debug)]
pub struct FilePath {
    os_path: PathBuf,
    path_flag: PathFlag,
    merge_path: Option<PathBuf>,
    env_path: Option<PathBuf>,
}

pub fn new_path(path: PathBuf) -> FilePath {
    let path_result = path_matchers::match_handle_path(&path);
    
    FilePath {
        os_path: path,
        path_flag: path_result.path_flag,
        merge_path: path_result.merge_path,
        env_path: None,
    }
}
