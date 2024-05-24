const COMMANDS: &[&str] = &["cmd_set_theme", "cmd_get_theme"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}