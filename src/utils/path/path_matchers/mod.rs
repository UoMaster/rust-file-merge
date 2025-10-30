mod core;

pub use core::match_src_function_dir;
use std::path::{Path, PathBuf};
use crate::utils::path::PathFlag;



pub struct MatchResult {
  pub path_flag: PathFlag,
  pub merge_path: Option<PathBuf>,
}

type PathMatcher = fn(&PathBuf) -> Option<MatchResult>;

pub fn match_handle_path(path: &PathBuf) -> MatchResult {
  let matchers: Vec<PathMatcher> = vec![match_src_function_dir];

  for matcher in matchers {
      if let Some(result) = matcher(&path) {
          return result;
      }
  }

  MatchResult {
      path_flag: PathFlag::NotFound,
      merge_path: None,
  }
}
