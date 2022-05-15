#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .menu(
      tauri::Menu::new()
        .add_item(tauri::CustomMenuItem::new("foo".to_string(), "foo"))
    )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
