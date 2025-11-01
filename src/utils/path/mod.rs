
use std::{env, fs, path::PathBuf};
use serde::Serialize;

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


#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum PathFlag {
    /// 根目录下的功能 例如 [components, hooks, pages, ...]
    RootFunction,
    /// src/core/{模块名称} 下的功能 例如：同上
    CoreFunction,
    /// env/{webid} 根目录下的功能 例如 [components, hooks, ...]
    EnvRootFunction,
    /// env/{webid}/core/{模块名称} 下的功能 例如：同上
    EnvCoreFunction,
    NotFound,
}

#[derive(Debug, Serialize)]
pub struct FilePath {
    /// 原始操作系统路径
    os_path: PathBuf,

    /// 判断 原始路径的 类型
    path_flag: PathFlag,

    /// 对应 merge 的 路径
    /// 所有 PathFlag 类型都应该有值
    merge_path: Option<PathBuf>,

    /// 对应 env 的 路径
    /// 只有 RootFunction 和 CoreFunction 类型应该有值
    env_path: Option<PathBuf>,

    /// env 路径的反转路径
    /// 只有 EnvRootFunction 和 EnvCoreFunction 类型应该有值
    env_reverse_path: Option<PathBuf>,
}

pub fn new_path(path: PathBuf) -> FilePath {
    let path_result = path_matchers::match_handle_path(&path);
    
    FilePath {
        os_path: path,
        path_flag: path_result.path_flag,
        merge_path: path_result.merge_path,
        env_path: path_result.env_path,
        env_reverse_path: None,
    }
}

