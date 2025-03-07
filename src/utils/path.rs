// https://github.com/DioxusLabs/dioxus/issues/1814
// https://github.com/tauri-apps/tauri/blob/f37e97d410c4a219e99f97692da05ca9d8e0ba3a/crates/tauri/scripts/core.js#L17
pub fn normalize_path(p: &str) -> String {
    #[cfg(target_os = "windows")]
    {
        let p = p.replace("\\", "/");
        format!("http://dioxus.localhost/{p}")
    }
    #[cfg(target_os = "macos")]
    {
        format!("{p}")
    }
}
