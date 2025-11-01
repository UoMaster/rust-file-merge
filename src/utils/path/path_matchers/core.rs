use std::path::{Component, PathBuf};
use crate::utils::path::{get_project_root_path, PathFlag};
use crate::utils::path::path_matchers::MatchResult;
use crate::utils::os::get_web_ip;
/**
 * 本文件是用作于匹配 src 下面的功能 目录/文件， 转化为 merge 路径和 env 路径
 * 例如：src/components/Button.vue 转化为 merge/components/Button.vue 和 env/{webid}/components/Button.vue
 */


const FUNCTION_DIR_NAMES: &[&str] = &[
    "components",
    "hooks",
    "store",
    "utils",
    "types",
    "layouts",
    "middleware",
    "plugins",
    "server",
    "static",
];


pub fn match_src_function_dir(path: &PathBuf) -> Option<MatchResult> {
    let src_path = get_project_root_path().join("src");
    
    // 用 Path API 提取相对路径，完全跨平台
    let relative_path = path.strip_prefix(&src_path).ok()?;
    
    // 获取第一个路径段（src 下的直接子目录）
    let mut components = relative_path.components();
    let first_component = components.next()?;
    
    // 检查是否是 Normal 组件（实际目录名）
    let function_dir = match first_component {
        Component::Normal(name) => name.to_str()?,
        _ => return None,
    };
    
    // 检查是否在我们的功能目录列表中
    if !FUNCTION_DIR_NAMES.contains(&function_dir) {
        return None;
    }
    
    // 获取剩余路径（function_dir 之后的部分）
    let remaining_path: PathBuf = components.collect();
    
    // 构建 merge 路径，完全用 PathBuf 操作
    let merge_path = src_path
        .join("merge")
        .join(function_dir)
        .join(&remaining_path);
    
    // 构建 env 路径
    let env_path = src_path
        .join("env")
        .join(get_web_ip())
        .join(function_dir)
        .join(&remaining_path);
    
    Some(MatchResult {
        path_flag: PathFlag::CoreFunction,
        merge_path: Some(merge_path),
        env_path: Some(env_path),
    })
}

