use anyhow::Result;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            if let Err(error) = setup(app) {
                eprintln!("{error}");
                Err(error.into())
            } else {
                Ok(())
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut tauri::App) -> Result<()> {
    let mut rgba = vec![];
    for _ in 0..32 * 32 {
        rgba.push(255);
        rgba.push(0);
        rgba.push(255);
        rgba.push(255);
    }
    let image = tauri::image::Image::new_owned(rgba, 32, 32);

    let menu = tauri::menu::SubmenuBuilder::new(app, "");
    let menu = menu.item(&tauri::menu::MenuItemBuilder::new("Item").enabled(false).build(app)?);
    let menu = menu.build()?;

    tauri::tray::TrayIconBuilder::new()
        .menu(&menu)
        .icon(image)
        .tooltip("Firezone")
        .build(app)?;
    Ok(())
}
