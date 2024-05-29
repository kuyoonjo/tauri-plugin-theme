#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::menu::{
    CheckMenuItemBuilder, MenuItemKind, PredefinedMenuItem, SubmenuBuilder, WINDOW_SUBMENU_ID,
};
use tauri_plugin_theme::{get_theme, set_theme, Theme};

fn main() {
    let mut ctx = tauri::generate_context!();
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_theme::init(ctx.config_mut()))
        .setup(|app| {
            if let Some(menu) = app.menu() {
                if let Some(MenuItemKind::Submenu(window)) = menu.get(WINDOW_SUBMENU_ID) {
                    let theme_menu = SubmenuBuilder::with_id(app, "theme", "Theme").build()?;
                    let theme_auto =
                        CheckMenuItemBuilder::with_id("theme_auto", "Auto").build(app)?;
                    let theme_light =
                        CheckMenuItemBuilder::with_id("theme_light", "Light").build(app)?;
                    let theme_dark =
                        CheckMenuItemBuilder::with_id("theme_dark", "Dark").build(app)?;
                    theme_auto.set_checked(false)?;
                    theme_light.set_checked(false)?;
                    theme_dark.set_checked(false)?;
                    theme_menu.append(&theme_auto)?;
                    theme_menu.append(&theme_light)?;
                    theme_menu.append(&theme_dark)?;
                    window.insert(&theme_menu, 2).unwrap();
                    let separator = PredefinedMenuItem::separator(app)?;
                    window.insert(&separator, 2).unwrap();

                    let theme = get_theme(app.handle());
                    match theme {
                        Theme::Light => {
                            theme_light.set_checked(true)?;
                        }
                        Theme::Dark => {
                            theme_dark.set_checked(true)?;
                        }
                        _ => {
                            theme_auto.set_checked(true)?;
                        }
                    }

                    app.on_menu_event(move |app, event| {
                        if event.id() == theme_auto.id() {
                            set_theme(app.clone(), Theme::Auto).unwrap();
                            theme_auto.set_checked(true).unwrap();
                            theme_light.set_checked(false).unwrap();
                            theme_dark.set_checked(false).unwrap();
                        } else if event.id() == theme_light.id() {
                            set_theme(app.clone(), Theme::Light).unwrap();
                            theme_auto.set_checked(false).unwrap();
                            theme_light.set_checked(true).unwrap();
                            theme_dark.set_checked(false).unwrap();
                        } else if event.id() == theme_dark.id() {
                            set_theme(app.clone(), Theme::Dark).unwrap();
                            theme_auto.set_checked(false).unwrap();
                            theme_light.set_checked(false).unwrap();
                            theme_dark.set_checked(true).unwrap();
                        }
                    });
                }
            }
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
