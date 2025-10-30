pub fn get_os_name() -> String {
    let os = std::env::consts::OS;
    os.to_string()
}

pub fn is_macos() -> bool {
    get_os_name() == "macos"
}

pub fn get_web_ip() -> String {
  // 临时 mock 的 web ip
  String::from("buaa")
}
