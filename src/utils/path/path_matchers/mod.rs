mod core;

pub use core::match_src_function_dir;
use crate::utils::path::PathFlag;



pub struct MatckResult {
  pub path_flag: PathFlag,
  pub merge_path: Option<String>,
}

type PathMatcher = fn(&str) -> Option<MatckResult>;

pub fn match_handle_path(path: String) -> MatckResult {
  let matchers: Vec<PathMatcher> = vec![match_src_function_dir];

  for matcher in matchers {
      if let Some(result) = matcher(&path) {
          return result;
      }
  }

  MatckResult {
      path_flag: PathFlag::NotFound,
      merge_path: None,
  }
}
