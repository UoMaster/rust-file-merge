use crate::utils::os::get_web_ip;
use crate::utils::path::path_matchers::MatchResult;
use crate::utils::path::{PathFlag, get_project_root_path};
use std::path::{Component, PathBuf};

pub trait PathResolver {
    fn get_project_root(&self) -> PathBuf;
    fn get_web_ip(&self) -> String;
}

pub struct DefaultPathResolver;

impl PathResolver for DefaultPathResolver {
    fn get_project_root(&self) -> PathBuf {
        get_project_root_path()
    }

    fn get_web_ip(&self) -> String {
        get_web_ip()
    }
}
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
    match_src_function_dir_with_resolver(path, &DefaultPathResolver)
}

pub fn match_src_function_dir_with_resolver(
    path: &PathBuf,
    resolver: &dyn PathResolver,
) -> Option<MatchResult> {
    let project_root = resolver.get_project_root();
    let src_path = project_root.join("src");

    let relative_path = path.strip_prefix(&src_path).ok()?;

    let mut components = relative_path.components();
    let first_component = components.next()?;

    let function_dir = match first_component {
        Component::Normal(name) => name.to_str()?,
        _ => return None,
    };

    if !FUNCTION_DIR_NAMES.contains(&function_dir) {
        return None;
    }

    let remaining_path: PathBuf = components.collect();

    let merge_path = src_path
        .join("merge")
        .join(function_dir)
        .join(&remaining_path);

    let env_path = src_path
        .join("env")
        .join(resolver.get_web_ip())
        .join(function_dir)
        .join(&remaining_path);

    Some(MatchResult {
        path_flag: PathFlag::CoreFunction,
        merge_path: Some(merge_path),
        env_path: Some(env_path),
        env_reverse_path: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::fs;
    use tempfile::TempDir;

    #[derive(Debug)]
    struct MockResolver {
        root: PathBuf,
        web_ip: String,
    }

      impl PathResolver for MockResolver {
        fn get_project_root(&self) -> PathBuf {
            self.root.clone()
        }

        fn get_web_ip(&self) -> String {
            self.web_ip.clone()
        }
    }
    #[derive(Debug)] 
    struct TestDirectory {
        button_file: PathBuf,
        root_dir: PathBuf,
        src_dir: PathBuf,
    }


    fn setup_test_directory() -> TestDirectory {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path().to_path_buf();

        let root_dir = root.join("user").join("project");
        
        let components_dir = root_dir.join("src").join("components").join("z");

        let button_file = components_dir.join("Button.vue");

        let src_dir = root_dir.join("src");
        TestDirectory {
            button_file,
            root_dir,
            src_dir,
        }
       
    }

    #[test]
    fn test_match_src_function_dir_with_real_files() {
        let test_directory = setup_test_directory();
       println!("test_directory: {:?}", test_directory);

        let mock_resolver = MockResolver {
            root: test_directory.root_dir.clone(),
            web_ip: "test-webid".to_string(),
        };


        let result = match_src_function_dir_with_resolver(&test_directory.button_file, &mock_resolver);


        assert_ne!(result, None);

        let file_path = result.unwrap();

        assert_eq!(file_path.path_flag, PathFlag::CoreFunction);
        assert_ne!(file_path.merge_path, None);
        assert_ne!(file_path.env_path, None);
        assert_eq!(file_path.env_reverse_path, None);

        /// merge/components/z/Button.vue
        let merge_path = test_directory.src_dir.join("merge").join("components").join("z").join("Button.vue");
        assert_eq!(file_path.merge_path, Some(merge_path));

        /// env/{webid}/components/z/Button.vue
        let env_path = test_directory.src_dir.join("env").join("test-webid").join("components").join("z").join("Button.vue");
        assert_eq!(file_path.env_path, Some(env_path));
    }

   
}
