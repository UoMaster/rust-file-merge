mod utils;

use notify::{RecommendedWatcher, RecursiveMode, Watcher, EventKind};
use std::path::Path;
use std::sync::mpsc::channel;
use crate::utils::path::get_current_path;

fn main() {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, notify::Config::default()).unwrap();

    let pwd = get_current_path();

    watcher
        .watch(Path::new(&pwd), RecursiveMode::Recursive)
        .unwrap();


    println!("开始监听: {:?}", pwd);
    loop {
        match rx.recv() {
            Ok(event) => match event {
                Ok(e) =>  {
                  println!("路径: {:?}", e.paths);
                  
                  match e.kind {
                    EventKind::Create(_) => println!("✅ 新增文件"),
                    EventKind::Modify(_) => println!("📝 修改文件"),
                    EventKind::Remove(_) => println!("🗑️  删除文件"),
                    EventKind::Access(_) => println!("👁️  访问文件"),
                    _ => println!("其他事件: {:?}", e.kind),
                  }
                },
                Err(_) => println!("Error: {:?}", event),
            },
            Err(_e) => (),
        }
    }
}
