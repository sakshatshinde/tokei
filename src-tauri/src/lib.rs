use libmpv2::Mpv;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust and Tauri!", name)
}

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
            app.manage(PlayerState(Mutex::new(None)));
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            pick_directory,
            init_player,
            play_media,
            toggle_pause
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct PlayerState(Mutex<Option<Mpv>>);

#[tauri::command]
async fn init_player(
    window: tauri::Window,
    state: tauri::State<'_, PlayerState>,
) -> Result<(), String> {
    let mpv = Mpv::new().map_err(|e| e.to_string())?;

    // Use higher-level methods to set properties
    mpv.set_property("vo", "gpu").map_err(|e| e.to_string())?;
    mpv.set_property("hwdec", "no").map_err(|e| e.to_string())?;
    // ! For some reason using this causes mpv to not load?
    // mpv.set_property("log-file", "internal_mpv.log")
    //     .map_err(|e| e.to_string())?;
    // mpv.set_property("v", "1").map_err(|e| e.to_string())?; // Verbose logging to help debug

    // Initialize player with video container (you may want to specify a container ID)
    // mpv.set_property("video-container", "videoContainer")
    //     .map_err(|e| e.to_string())?;

    // Pass window handle (same as before)

    #[cfg(target_os = "windows")]
    let handle = {
        let hwnd = window.hwnd().map_err(|e| e.to_string())?;
        hwnd.0 as *mut std::ffi::c_void as i64
    };

    #[cfg(target_os = "linux")]
    let handle = window.xid().map_err(|e| e.to_string())? as i64;

    #[cfg(target_os = "macos")]
    let handle = unsafe {
        let ns_window = window.ns_window().map_err(|e| e.to_string())?;
        ns_window as i64
    };

    mpv.set_property("wid", handle).map_err(|e| e.to_string())?;
    // mpv.set_property("wid", container_id)
    //     .map_err(|e| e.to_string())?;

    *state.0.lock().unwrap() = Some(mpv);
    Ok(())
}

#[tauri::command]
async fn play_media(path: String, state: tauri::State<'_, PlayerState>) -> Result<(), String> {
    let guard = state.0.lock().unwrap();
    let mpv = guard.as_ref().ok_or("MPV not initialized")?;

    mpv.command("loadfile", &[&path])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn toggle_pause(state: tauri::State<'_, PlayerState>) -> Result<(), String> {
    let guard = state.0.lock().unwrap();
    let mpv = guard.as_ref().ok_or("MPV not initialized")?;
    let paused: bool = mpv.get_property("pause").map_err(|e| e.to_string())?;

    mpv.set_property("pause", !paused)
        .map_err(|e| e.to_string())?;

    Ok(())
}
