use std::env;

pub fn get_project_root() -> Option<String> {
    let mut dir = env::current_dir().ok()?;
    while dir.parent().is_some() {
        if dir.join("Cargo.toml").exists() {
            return dir.to_str().map(|s| s.to_string());
        }
        dir = dir.parent()?.to_path_buf();
    }
    None
}

pub fn get_project_source() -> Option<String> {
    let root = get_project_root()?;
    Some(format!("{}/src", root))
}
