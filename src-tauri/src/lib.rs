use libmpv2::Mpv;
use std::sync::{Arc, Mutex};
use tauri::{Manager, Window};
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
            app.manage(PlayerState(Mutex::new(None).into()));
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

struct PlayerState(Arc<Mutex<Option<Mpv>>>);

#[tauri::command]
async fn init_player(
    app: tauri::AppHandle,
    state: tauri::State<'_, PlayerState>,
) -> Result<(), String> {
    let video_window = Window::builder(&app, "mpv_window")
        .title("mpv")
        .focused(true)
        // .fullscreen(true)
        // .shadow(false)
        .build()
        .map_err(|e| e.to_string())?;

    #[cfg(target_os = "windows")]
    let handle = {
        let hwnd = video_window.hwnd().map_err(|e| e.to_string())?;
        hwnd.0 as *mut std::ffi::c_void as i64
    };

    #[cfg(target_os = "linux")]
    let handle = video_window.xid().map_err(|e| e.to_string())? as i64;

    #[cfg(target_os = "macos")]
    let handle = unsafe {
        let ns_window = video_window.ns_window().map_err(|e| e.to_string())?;
        ns_window as i64
    };

    let mpv = Mpv::new().map_err(|e| e.to_string())?;
    mpv.set_property("wid", handle).map_err(|e| e.to_string())?;

    let state_clone = state.0.clone(); // Clone the Arc for use in the closure
    let close_handler = move |event: &tauri::WindowEvent| {
        match event {
            tauri::WindowEvent::CloseRequested { .. } => {
                if let Ok(mut guard) = state_clone.lock() {
                    if let Some(mpv) = guard.take() {
                        let _ = mpv.command("quit", &[]);
                    }
                }
            }
            _ => {}
        };
    };

    video_window.on_window_event(close_handler);

    // Enable ALL UI elements
    mpv.set_property("osc", "yes").map_err(|e| e.to_string())?;

    mpv.set_property("osd-bar", "yes")
        .map_err(|e| e.to_string())?;
    mpv.set_property("osd-level", "2")
        .map_err(|e| e.to_string())?;

    // Input settings
    mpv.set_property("input-default-bindings", "yes")
        .map_err(|e| e.to_string())?;
    mpv.set_property("input-vo-keyboard", "yes")
        .map_err(|e| e.to_string())?;

    mpv.set_property("vo", "gpu-next")
        .map_err(|e| e.to_string())?;
    mpv.set_property("hwdec", "no").map_err(|e| e.to_string())?;

    // ! For some reason using this causes mpv to not load?
    mpv.set_property("log-file", "internal_mpv.log")
        .map_err(|e| e.to_string())?;

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

    mpv.set_property("osc", "yes").map_err(|e| e.to_string())?;

    Ok(())
}
