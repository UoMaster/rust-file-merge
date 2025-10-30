use crate::utils::os::{is_macos};
use std::{env, fs, path::PathBuf};

mod path_matchers;



pub fn get_current_path() -> PathBuf {
    env::current_dir().unwrap().join("demo")
}

pub fn get_project_root_path() -> PathBuf {
    get_current_path()
}

pub fn get_split_key() -> &'static str {
    if is_macos() { "/" } else { "\\" }
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
    pre_path: String,
    os_path: String,
    split_path: Vec<String>,
    path_flag: PathFlag,
    merge_path: Option<String>,
}

trait PathTrait {
    fn get_pre_path(&self) -> String;
    fn get_os_path(&self) -> String;
    fn get_split_path(&self) -> Vec<String>;
}

impl PathTrait for FilePath {
    fn get_pre_path(&self) -> String {
        self.pre_path.clone()
    }
    fn get_os_path(&self) -> String {
        self.os_path.clone()
    }
    fn get_split_path(&self) -> Vec<String> {
        self.split_path.clone()
    }
}

pub fn new_path(path: String) -> FilePath {
    let split_key = get_split_key();
    let os_path = path.replace(split_key, "/");

    let mut file_path = FilePath {
        pre_path: path.clone(),
        os_path,
        split_path: path.split(split_key).map(|s| s.to_string()).collect(),
        path_flag: PathFlag::NotFound,
        merge_path: None,
    };

    let path_result = path_matchers::match_handle_path(path);
    file_path = FilePath {
        path_flag: path_result.path_flag,
        merge_path: path_result.merge_path,
        ..file_path
    };


    file_path
}
