use libmpv2::Mpv;
use std::sync::{Arc, Mutex};
use tauri::{Manager, Url};
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
fn pick_directory(app_handle: tauri::AppHandle) -> Option<String> {
    let (tx, rx) = std::sync::mpsc::channel();
    app_handle.dialog().file().pick_folder(move |f| {
        if let Some(path) = f {
            let _ = tx.send(path.to_string());
        }
    });
    rx.recv().ok()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(PlayerState(Mutex::new(None).into()));
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            pick_directory,
            init_player,
            quit_player,
            watch_player_shutdown,
            create_anilist_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct PlayerState(Arc<Mutex<Option<Mpv>>>);

impl Drop for PlayerState {
    fn drop(&mut self) {
        if let Ok(mut guard) = self.0.lock() {
            if let Some(mpv) = guard.take() {
                let _ = mpv.command("quit", &[]);
            }
        }
    }
}

#[tauri::command]
async fn init_player(path: String, state: tauri::State<'_, PlayerState>) -> Result<(), String> {
    let mpv = Mpv::new().map_err(|e| e.to_string())?;

    // On Screen controls
    mpv.set_property("osc", "yes").map_err(|e| e.to_string())?;

    // Input settings
    mpv.set_property("input-default-bindings", "yes")
        .map_err(|e| e.to_string())?;
    mpv.set_property("input-vo-keyboard", "yes")
        .map_err(|e| e.to_string())?;
    mpv.set_property("input-cursor", "yes")
        .map_err(|e| e.to_string())?;

    // GPU
    mpv.set_property("vo", "gpu-next")
        .map_err(|e| e.to_string())?;
    mpv.set_property("hwdec", "no").map_err(|e| e.to_string())?;

    mpv.set_property("log-file", "internal_mpv.log")
        .map_err(|e| e.to_string())?;

    let quoted_path = format!("\"{}\"", path);
    mpv.command("loadfile", &[&quoted_path, "append-play"])
        .map_err(|e| e.to_string())?;

    let mut guard = state.0.lock().unwrap();
    *guard = Some(mpv);

    Ok(())
}

#[tauri::command]
async fn quit_player(state: tauri::State<'_, PlayerState>) -> Result<(), String> {
    if let Ok(mut guard) = state.0.lock() {
        if let Some(mpv) = guard.take() {
            let _ = mpv.command("quit", &[]); // Explicitly quit MPV
            return Ok(());
        }
    }
    Err("MPV cleanup failed".to_string())
}

#[tauri::command]
async fn watch_player_shutdown(state: tauri::State<'_, PlayerState>) -> Result<(), String> {
    // Initial check for MPV existence
    {
        let guard = state.0.lock().unwrap();
        if guard.as_ref().is_none() {
            return Err("MPV state check ran before mpv was launched".to_string());
        }
    }

    loop {
        let should_shutdown = {
            let mut guard = state.0.lock().unwrap();
            if let Some(ref mut mpv) = *guard {
                if let Some(event) = mpv.event_context_mut().wait_event(0.0) {
                    match event.unwrap() {
                        libmpv2::events::Event::Shutdown => true,
                        _ => false,
                    }
                } else {
                    false
                }
            } else {
                return Ok(()); // MPV was cleaned up elsewhere
            }
        };

        if should_shutdown {
            if let Ok(mut guard) = state.0.lock() {
                if let Some(mpv) = guard.take() {
                    let _ = mpv.command("quit", &[]);
                }
            }
            return Ok(());
        }
    }
}

#[tauri::command]
async fn create_anilist_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    let _anilist_window = tauri::WebviewWindowBuilder::new(
        &app_handle,
        "anilist_window".to_string(),
        tauri::WebviewUrl::External(Url::parse("https://anilist.co/home").unwrap()),
    )
    .min_inner_size(1280., 720.)
    .title("Tokei - Anilist")
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

// #[tauri::command]
// async fn create_subsplease_window(app_handle: tauri::AppHandle) -> Result<(), String> {
//     let _subsplease_window = tauri::WebviewWindowBuilder::new(
//         &app_handle,
//         "subsplease_window".to_string(),
//         tauri::WebviewUrl::External(Url::parse("https://subsplease.org/").unwrap()),
//     )
//     .min_inner_size(1280., 720.)
//     .title("Tokei - Subsplease")
//     .build()
//     .map_err(|e| e.to_string())?;

//     Ok(())
// }

// #[tauri::command]
// async fn create_nyaa_window(app_handle: tauri::AppHandle) -> Result<(), String> {
//     let _nyaa_window = tauri::WebviewWindowBuilder::new(
//         &app_handle,
//         "nyaa_window".to_string(),
//         tauri::WebviewUrl::External(Url::parse("https://nyaa.si/").unwrap()),
//     )
//     .min_inner_size(1280., 720.)
//     .title("Tokei - Nyaa :3")
//     .build()
//     .map_err(|e| e.to_string())?;

//     Ok(())
// }

// async fn return_anime_list() {
//     todo!()
// }
