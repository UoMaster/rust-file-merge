use super::os::{is_macos, get_web_ip};
use std::{env::{self, join_paths}, fs, path::PathBuf};


pub fn get_current_path() -> PathBuf {
  env::current_dir().unwrap().join("demo")
}

pub fn get_project_root_path() -> PathBuf {
  get_current_path()
}

pub fn get_split_key() -> &'static str {
    if is_macos() {  "/" } else {  "\\" }
}

pub fn get_core_module_names() -> Vec<String> {
  let core_dir_path = get_project_root_path().join("src").join("core");
  let mut core_module_names = Vec::new();
  
  for entry in match fs::read_dir(core_dir_path) {
    Ok(entries) => entries,
    Err(_) => return Vec::new(),
  } {
    match entry {
      Ok(entry) => {
        let path = entry.path();
        if path.is_dir() {
          core_module_names.push(path.to_str().unwrap().to_string());
        }
      },
      Err(_) => continue,
    }
  }
  core_module_names
}

#[derive(Debug)]
enum PathFlag {
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
    flag: PathFlag,
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

    println!("os_path: {:?}", os_path);
    FilePath {
        pre_path: path.clone(),
        os_path,
        split_path: path.split(split_key).map(|s| s.to_string()).collect(),
        flag: PathFlag::NotFound,
    }
}

#[cfg(test)]
mod file_path_tests {
    use super::*;
    #[test]
    fn test_new_path_for_macos() {
        let path = new_path("src/utils/path.rs".to_string());
        assert_eq!(path.get_pre_path(), "src/utils/path.rs".to_string());
        assert_eq!(path.get_os_path(), "src/utils/path.rs".to_string());
        assert_eq!(path.get_split_path(), vec!["src", "utils", "path.rs"]);
    }
    #[test]
    #[ignore = "测试时需要修改 get_split_key() 的返回值"]
    fn test_new_path_for_windows() {
      // 测试时需要修改 get_split_key() 的返回值
        let path = new_path(
            "C:\\Users\\wuhongbin\\Desktop\\rust-file-merge\\src\\utils\\path.rs".to_string(),
        );

        assert_eq!(
            path.get_pre_path(),
            "C:\\Users\\wuhongbin\\Desktop\\rust-file-merge\\src\\utils\\path.rs".to_string()
        );

        assert_eq!(
            path.get_os_path(),
            "C:/Users/wuhongbin/Desktop/rust-file-merge/src/utils/path.rs".to_string()
        );
        assert_eq!(
            path.get_split_path(),
            vec![
                "C:",
                "Users",
                "wuhongbin",
                "Desktop",
                "rust-file-merge",
                "src",
                "utils",
                "path.rs"
            ]
        );
    }
}


#[cfg(test)]
mod get_core_module_names_tests {
    use super::*;
    use insta::assert_debug_snapshot;
    #[test]
    fn mock_get_core_module_names() {
        let core_module_names = get_core_module_names();
        assert_debug_snapshot!(core_module_names);
    }
}