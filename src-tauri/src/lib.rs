use libmpv2::Mpv;
use std::sync::{Arc, Mutex};
use tauri::Manager;
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
            play_media,
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
async fn init_player(state: tauri::State<'_, PlayerState>) -> Result<(), String> {
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

    // ! Currently this leaves a dangling pointer when mpv window is closed
    *state.0.lock().unwrap() = Some(mpv);

    Ok(())
}

#[tauri::command]
async fn play_media(path: String, state: tauri::State<'_, PlayerState>) -> Result<(), String> {
    let guard = state.0.lock().unwrap();
    let mpv = guard.as_ref().ok_or("MPV not initialized")?;

    let quoted_path = format!("\"{}\"", path);

    mpv.command("loadfile", &[&quoted_path, "append-play"])
        .map_err(|e| e.to_string())?;

    Ok(())
}
