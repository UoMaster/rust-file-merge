use regex::Regex;
use std::sync::LazyLock;
use crate::utils::path::{get_project_root_path, PathFlag};
use crate::utils::path::path_matchers::MatckResult;

const FUNCTION_DIR_NAMES: &[&str] = &[
    "components",
    "hooks",
    "store",
    "utils",
    "types",
    "plugins",
    "layouts",
    "middleware",
    "plugins",
    "server",
    "static",
];

static FUNCTION_DIR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let src_path = get_project_root_path().join("src");
    let src_path_str = src_path.to_str().expect("Invalid src path");
    let pattern = format!(
        r"{}/({})/",
        regex::escape(src_path_str),
        FUNCTION_DIR_NAMES.join("|")
    );
    Regex::new(&pattern).expect("Invalid regex pattern")
});

pub fn match_src_function_dir(path: &str) -> Option<MatckResult> {
    let src_path = get_project_root_path().join("src");
    
    let captures = FUNCTION_DIR_REGEX.captures(path)?;

    let function_dir = captures.get(1)?.as_str();
    let relative_path = path.strip_prefix(src_path.to_str()?)?;
    let merge_path = src_path
        .join("merge")
        .join(function_dir)
        .join(&relative_path[function_dir.len() + 2..]);

    Some(MatckResult {
        path_flag: PathFlag::CoreFunction,
        merge_path: Some(merge_path.to_str()?.to_string()),
    })
}

