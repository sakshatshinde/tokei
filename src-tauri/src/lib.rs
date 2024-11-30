use jwalk::WalkDir;
use libmpv2::Mpv;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};
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
            toggle_webview,
            create_child_webview,
            wrap_read_media_directory_structure
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
fn toggle_webview(
    app_handle: tauri::AppHandle,
    webview_label: String,
    toggle_mode: String,
) -> Result<(), String> {
    let webview = app_handle
        .get_webview(&webview_label)
        .ok_or_else(|| format!("Webview {} not found", webview_label))?;

    match toggle_mode.as_str() {
        "hide" => webview.hide().map_err(|e| e.to_string())?,
        "show" => webview.show().map_err(|e| e.to_string())?,
        _ => (),
    }

    Ok(())
}

#[tauri::command]
async fn create_child_webview(
    app_handle: tauri::AppHandle,
    service_name: String,
    service_url: String,
) -> Result<(), String> {
    let main_window = app_handle.get_window("main").unwrap();
    let main_window_size = main_window.outer_size().map_err(|e| e.to_string())?;

    // creates a child webview with the name eg. anilist_webview
    let webview_name = format!("{}_webview", service_name);

    let already_exists = main_window.get_webview(&webview_name);
    if already_exists.is_some() {
        return Ok(());
    }

    let _child_webview = main_window
        .add_child(
            tauri::webview::WebviewBuilder::new(
                webview_name,
                tauri::WebviewUrl::External(Url::parse(&service_url).unwrap()),
            )
            .auto_resize(),
            tauri::LogicalPosition::new(60., 0.),
            tauri::LogicalSize::new(main_window_size.width - 60, main_window_size.height),
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct MediaDirectoryStructure {
    name: String,
    path: PathBuf,
    files: Vec<String>,
    subdirectories: Vec<MediaDirectoryStructure>,
}

impl MediaDirectoryStructure {
    fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            files: Vec::new(),
            subdirectories: Vec::new(),
        }
    }
}

#[tauri::command]
async fn wrap_read_media_directory_structure(
    root_anime_path: String,
) -> Result<MediaDirectoryStructure, String> {
    let result = read_media_directory_structure(root_anime_path.into());
    return result;
}

/// Recursively scans the specified root directory and constructs a hierarchical structure of directories and media files.
///
/// **Warning this is AI generated code**
///
/// This function uses the `WalkDir` crate to traverse the directory tree starting from the given root directory (`root_anime_path`). It collects all relevant media files (filtered by their extensions) and subdirectories, organizing them into a `MediaDirectoryStructure` object. The function returns this structure in a nested fashion.
///
/// The function supports only files with video file extensions defined by the `is_video_file` helper function. Any non-video files are ignored during the scan.
///
/// # Parameters
///
/// - `root_anime_path` (`PathBuf`): The path to the root directory that will be scanned for media files and subdirectories. This is the top-level directory from which the directory scan begins.
///
/// # Return Value
///
/// - `Result<MediaDirectoryStructure, String>`:
///   - **`Ok(MediaDirectoryStructure)`**: The function returns the constructed `MediaDirectoryStructure` object, which contains the entire directory structure starting from the root path, including all media files and subdirectories.
///   - **`Err(String)`**: If an error occurs (e.g., issues with reading directories, file system errors, or unsupported file types), an error message is returned as a `String`.
///   
/// # Example
///
/// ```rust
/// let root_dir = PathBuf::from("/path/to/media/directory");
/// let result = read_media_directory_structure(root_dir);
///
/// match result {
///     Ok(structure) => {
///         println!("Successfully read directory structure!");
///     },
///     Err(e) => {
///         println!("Error reading directory structure: {}", e);
///     }
/// }
/// ```
fn read_media_directory_structure(
    root_anime_path: PathBuf,
) -> Result<MediaDirectoryStructure, String> {
    // ! Don't want to deal with Pinned Box futures so not directly using this as a tauri command as the size can recursively grow.

    let root_name = root_anime_path
        .file_name()
        .map(|os_str| os_str.to_string_lossy().to_string())
        .unwrap_or_else(|| root_anime_path.to_string_lossy().to_string());

    let mut root_structure = MediaDirectoryStructure::new(root_name, root_anime_path.clone());
    let mut subdirectory_map: HashMap<PathBuf, MediaDirectoryStructure> = HashMap::new();

    subdirectory_map.insert(
        root_anime_path.clone(),
        MediaDirectoryStructure::new(
            root_anime_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            root_anime_path.clone(),
        ),
    );

    for result in WalkDir::new(&root_anime_path)
        .sort(true)
        .follow_links(true)
        .into_iter()
    {
        let entry = result.map_err(|e| e.to_string())?;
        let entry_path = entry.path().to_path_buf();

        // Skip hidden directories and files
        let entry_name = entry.file_name().to_string_lossy();
        if entry_name.starts_with(".") {
            continue;
        }

        if entry.file_type().is_dir() {
            let dir_structure = MediaDirectoryStructure::new(
                entry.file_name().to_string_lossy().to_string(),
                entry_path.clone(),
            );
            subdirectory_map.insert(entry_path.clone(), dir_structure);
        } else if entry.file_type().is_file() {
            let file_extension = entry_path.extension().unwrap();

            if !is_video_file(file_extension.to_string_lossy().to_string().as_str()) {
                continue;
            }

            if let Some(parent_path) = entry_path.parent() {
                if let Some(parent_dir) = subdirectory_map.get_mut(parent_path) {
                    parent_dir
                        .files
                        .push(entry.file_name().to_string_lossy().to_string());
                }
            }
        }
    }

    for (path, subdir) in subdirectory_map.clone() {
        if let Some(parent_path) = path.parent() {
            if let Some(parent_dir) = subdirectory_map.get_mut(parent_path) {
                parent_dir.subdirectories.push(subdir);
            }
        }
    }

    if let Some(root_dir) = subdirectory_map.get(&root_anime_path) {
        root_structure.subdirectories = root_dir.subdirectories.clone();
        root_structure.files = root_dir.files.clone();
    }

    Ok(root_structure)
}

fn is_video_file(extension: &str) -> bool {
    vec![
        "mp4", "avi", "mov", "mkv", "wmv", "flv", "webm", "vob", "ogv", "m4v", "3gp", "3g2",
    ]
    .contains(&extension)
}
