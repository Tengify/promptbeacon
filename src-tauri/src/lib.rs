pub mod bridge;
mod detection;
mod model;
mod platform;
mod storage;
mod worker;

use model::{BeaconState, Bounds, Config};
use platform::PlatformDetector;
use serde::Serialize;
use std::{
    sync::Mutex,
    time::{Duration, Instant},
};
use tauri::{Emitter, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

struct Runtime {
    config: Mutex<Config>,
    beacon: Mutex<BeaconState>,
    preview: Mutex<Option<(Instant, BeaconState)>>,
    worker: Mutex<Option<worker::Worker>>,
    pending: Mutex<(bool, Option<BeaconState>)>,
    permission: Mutex<bool>,
    settings_warning: Mutex<Option<String>>,
    diagnostics: Mutex<Option<detection::ScanDiagnostics>>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AppStatus {
    config: Config,
    beacon: BeaconState,
    platform: String,
    permission: bool,
    settings_warning: Option<String>,
    integration: String,
    data_path: String,
    version: String,
}
fn settings_only(window: &WebviewWindow) -> Result<(), String> {
    if window.label() == "main" {
        Ok(())
    } else {
        Err("This command belongs to the settings window.".into())
    }
}
#[tauri::command]
fn get_status(window: WebviewWindow, state: tauri::State<Runtime>) -> Result<AppStatus, String> {
    settings_only(&window)?;
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    #[cfg(windows)]
    let command = format!("\"{}\" --statusline --terminal-title", exe.display());
    #[cfg(not(windows))]
    let command = format!(
        "'{}' --statusline --terminal-title",
        exe.display().to_string().replace('\'', "'\\''")
    );
    let integration = serde_json::to_string_pretty(
        &serde_json::json!({"statusLine":{"type":"command","command":command,"refreshInterval":1}}),
    )
    .map_err(|e| e.to_string())?;
    // End each lock scope before acquiring the next mutex.
    let config = state.config.lock().map_err(|e| e.to_string())?.clone();
    let beacon = state.beacon.lock().map_err(|e| e.to_string())?.clone();
    Ok(AppStatus {
        config,
        beacon,
        platform: std::env::consts::OS.into(),
        permission: *state.permission.lock().map_err(|e| e.to_string())?,
        settings_warning: state
            .settings_warning
            .lock()
            .map_err(|e| e.to_string())?
            .clone(),
        integration,
        data_path: storage::data_dir()?.display().to_string(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}
#[tauri::command]
fn get_detection_diagnostics(
    window: WebviewWindow,
    state: tauri::State<Runtime>,
) -> Result<serde_json::Value, String> {
    settings_only(&window)?;
    Ok(
        serde_json::json!({"version":env!("CARGO_PKG_VERSION"),"platform":std::env::consts::OS,
        "permissionGranted":*state.permission.lock().map_err(|e|e.to_string())?,
        "lastScan":state.diagnostics.lock().map_err(|e|e.to_string())?.clone()}),
    )
}
#[tauri::command]
fn get_beacon(state: tauri::State<Runtime>) -> serde_json::Value {
    let beacon = state
        .beacon
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone();
    let config = state
        .config
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone();
    serde_json::json!({"beacon":beacon,"config":config})
}

#[tauri::command]
fn save_settings(
    window: WebviewWindow,
    config: Config,
    state: tauri::State<Runtime>,
) -> Result<(), String> {
    settings_only(&window)?;
    let mut current = state.config.lock().map_err(|e| e.to_string())?;
    storage::save_config(&config)?;
    *current = config;
    *state.settings_warning.lock().map_err(|e| e.to_string())? = None;
    Ok(())
}
#[tauri::command]
fn request_accessibility(window: WebviewWindow) -> Result<(), String> {
    settings_only(&window)?;
    platform::Detector::new().request_permission();
    Ok(())
}
#[tauri::command]
fn preview_beacon(
    window: WebviewWindow,
    model: String,
    effort: Option<String>,
    state: tauri::State<Runtime>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    settings_only(&window)?;
    if model.len() > 120 || model.chars().any(char::is_control) {
        return Err("Invalid preview model.".into());
    }
    let effort = effort
        .map(|e| detection::effort_label(&e))
        .filter(|e| e != "Unknown");
    let pos = window.outer_position().map_err(|e| e.to_string())?;
    let size = window.outer_size().map_err(|e| e.to_string())?;
    let config = state.config.lock().map_err(|e| e.to_string())?.clone();
    let beacon = BeaconState {
        status: "preview".into(),
        application: Some("PromptBeacon preview".into()),
        color: config.color_for(Some(&model)),
        model: Some(model),
        effort,
        source: "preview".into(),
        message: "Preview only. This is not a detected session. Stops after 12 seconds.".into(),
        bounds: Some(Bounds {
            x: pos.x as f64,
            y: pos.y as f64,
            width: size.width as f64,
            height: size.height as f64,
            logical: false,
        }),
        preview: true,
    };
    *state.preview.lock().map_err(|e| e.to_string())? =
        Some((Instant::now() + Duration::from_secs(12), beacon.clone()));
    publish(&app, beacon, &config);
    Ok(())
}
#[tauri::command]
fn stop_preview(
    window: WebviewWindow,
    state: tauri::State<Runtime>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    settings_only(&window)?;
    *state.preview.lock().map_err(|e| e.to_string())? = None;
    let config = state.config.lock().map_err(|e| e.to_string())?.clone();
    publish(
        &app,
        BeaconState::idle("inactive", "Waiting for a fresh selection."),
        &config,
    );
    Ok(())
}

fn overlay(app: &tauri::AppHandle, beacon: &BeaconState, config: &Config) -> Result<(), String> {
    let Some(window) = app.get_webview_window("beacon") else {
        return Err("Beacon window is unavailable.".into());
    };
    if (!config.enabled && !beacon.preview)
        || ["inactive", "permission", "error", "paused"].contains(&beacon.status.as_str())
    {
        return window.hide().map_err(|e| e.to_string());
    }
    if let Some(b) = &beacon.bounds {
        if b.logical {
            window
                .set_position(tauri::LogicalPosition::new(b.x, b.y))
                .map_err(|e| e.to_string())?;
            window
                .set_size(tauri::LogicalSize::new(b.width, b.height))
                .map_err(|e| e.to_string())?;
        } else {
            window
                .set_position(tauri::PhysicalPosition::new(b.x as i32, b.y as i32))
                .map_err(|e| e.to_string())?;
            window
                .set_size(tauri::PhysicalSize::new(b.width as u32, b.height as u32))
                .map_err(|e| e.to_string())?;
        }
    } else {
        // Wayland compositors control placement. The label remains useful even without a border.
        window
            .set_size(tauri::LogicalSize::new(370., 84.))
            .map_err(|e| e.to_string())?;
    }
    window
        .emit("beacon-appearance", config)
        .map_err(|e| e.to_string())?;
    window
        .emit("beacon-state", beacon)
        .map_err(|e| e.to_string())?;
    window.show().map_err(|e| e.to_string())
}
fn publish(app: &tauri::AppHandle, mut beacon: BeaconState, config: &Config) {
    let state = app.state::<Runtime>();
    *state.beacon.lock().unwrap_or_else(|e| e.into_inner()) = beacon.clone();
    if let Err(error) = overlay(app, &beacon, config) {
        beacon.message = format!("Beacon could not be shown: {error}");
        beacon.status = "error".into();
        if let Some(window) = app.get_webview_window("beacon") {
            let _ = window.hide();
        }
    }
    if let Some(tray) = app.tray_by_id("promptbeacon") {
        let tooltip = match &beacon.model {
            Some(model) => format!(
                "PromptBeacon: {model} · {}",
                beacon.effort.as_deref().unwrap_or("Effort unavailable")
            ),
            None => format!("PromptBeacon: {}", beacon.status),
        };
        let _ = tray.set_tooltip(Some(&tooltip));
    }
    *state.beacon.lock().unwrap_or_else(|e| e.into_inner()) = beacon.clone();
    let _ = app.emit_to("main", "beacon-state", &beacon);
}
fn show_settings(app: &tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}
// Coalesce background updates to one queued main-thread operation. The detector
// never dispatches synchronous window calls, so joining it on exit cannot deadlock.
fn queue_publish(app: &tauri::AppHandle, beacon: BeaconState) {
    let state = app.state::<Runtime>();
    let mut pending = state.pending.lock().unwrap_or_else(|e| e.into_inner());
    pending.1 = Some(beacon);
    if pending.0 {
        return;
    }
    pending.0 = true;
    drop(pending);
    let handle = app.clone();
    if app
        .run_on_main_thread(move || {
            let state = handle.state::<Runtime>();
            let candidate = {
                let mut pending = state.pending.lock().unwrap_or_else(|e| e.into_inner());
                pending.0 = false;
                pending.1.take()
            };
            if let Some(mut beacon) = candidate {
                let config = state
                    .config
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .clone();
                if let Some((until, preview)) =
                    &*state.preview.lock().unwrap_or_else(|e| e.into_inner())
                {
                    if *until > Instant::now() {
                        beacon = preview.clone();
                    } else if beacon.preview {
                        beacon = BeaconState::idle("inactive", "Waiting for a fresh selection.");
                    }
                } else if beacon.preview {
                    beacon = BeaconState::idle("inactive", "Waiting for a fresh selection.");
                }
                beacon.color = config.color_for(beacon.model.as_deref());
                publish(&handle, beacon, &config);
            }
        })
        .is_err()
    {
        let mut pending = state.pending.lock().unwrap_or_else(|e| e.into_inner());
        *pending = (false, None);
    }
}
fn monitor(app: tauri::AppHandle) -> std::io::Result<worker::Worker> {
    worker::Worker::spawn(move |stop| {
        let mut detector = platform::Detector::new();
        let mut previous: Option<(BeaconState, Config)> = None;
        let mut last_diagnostic_key = String::new();
        let mut last_prune = Instant::now();
        bridge::prune_expired();
        while !stop.requested() {
            let state = app.state::<Runtime>();
            let config = state
                .config
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .clone();
            let permission = detector.permission();
            *state.permission.lock().unwrap_or_else(|e| e.into_inner()) = permission;
            let preview = state
                .preview
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .as_ref()
                .filter(|(until, _)| *until > Instant::now())
                .map(|(_, b)| b.clone());
            let mut beacon = if let Some(preview) = preview {
                preview
            } else if !config.enabled {
                BeaconState::idle(
                    "paused",
                    "Beacon paused. Resume from settings or the tray menu.",
                )
            } else {
                match detector.probe() {
                    Ok(Some(observation)) if observation.terminal => {
                        if let Some(mut beacon) = bridge::read_title(&observation.title, &config) {
                            beacon.bounds = observation.bounds;
                            beacon
                        } else {
                            let mut b = BeaconState::idle("unknown", "No fresh Claude Code status line is linked to this terminal. See Connections.");
                            b.application = Some(observation.application);
                            b.bounds = observation.bounds;
                            b
                        }
                    }
                    Ok(Some(observation)) => detection::from_observation(observation, &config),
                    Ok(None) => BeaconState::idle(
                        "inactive",
                        "Open a Codex or Claude Code session to see its selection.",
                    ),
                    Err(error) => {
                        BeaconState::idle(if permission { "error" } else { "permission" }, error)
                    }
                }
            };
            if stop.requested() {
                break;
            }
            if let Some(report) = detector.diagnostics() {
                let key = report.stable_key();
                *state.diagnostics.lock().unwrap_or_else(|e| e.into_inner()) = Some(report.clone());
                if key != last_diagnostic_key {
                    let payload = serde_json::json!({"version":env!("CARGO_PKG_VERSION"),"platform":std::env::consts::OS,"permissionGranted":permission,"lastScan":report});
                    if let Ok(dir) = storage::data_dir() {
                        let _ = storage::create_private_dir(&dir);
                        let _ = storage::atomic_write(
                            &dir.join("detection-diagnostics.json"),
                            payload.to_string().as_bytes(),
                        );
                    }
                    last_diagnostic_key = key;
                }
                if beacon.source == "accessibility" && beacon.model.is_none() && !report.completed {
                    beacon.message = "The composer scan reached its work limit. Focus the prompt field and try again; scan details are available in Connections.".into();
                }
            }
            if previous.as_ref() != Some(&(beacon.clone(), config.clone())) {
                queue_publish(&app, beacon.clone());
                previous = Some((beacon, config.clone()));
            }
            if last_prune.elapsed() >= Duration::from_secs(60) {
                bridge::prune_expired();
                last_prune = Instant::now();
            }
            if stop.wait(Duration::from_millis(if config.enabled {
                config.poll_ms
            } else {
                1500
            })) {
                break;
            }
        }
    })
}

pub fn run() {
    if std::env::args().any(|a| a == "--statusline") {
        if let Err(error) = bridge::run() {
            eprintln!("PromptBeacon status line: {error}");
            std::process::exit(1);
        }
        return;
    }
    if std::env::args().any(|a| a == "--diagnostics") {
        let result = storage::data_dir().and_then(|dir| {
            storage::read_limited(&dir.join("detection-diagnostics.json"), 65536)
                .and_then(|bytes| String::from_utf8(bytes).map_err(|e| e.to_string()))
                .map_err(|e| format!("No saved scan diagnostics yet: {e}"))
        });
        match result {
            Ok(json) => println!("{json}"),
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        return;
    }
    if std::env::args().any(|a| a == "--probe") {
        let mut detector = platform::Detector::new();
        match detector.probe() {
            Ok(Some(o)) => {
                let count = o.controls.len();
                let s = detection::from_observation(o, &Config::default());
                println!(
                    "{}",
                    serde_json::json!({"beacon":s,"selectionControlCount":count})
                );
            }
            Ok(None) => println!("{{\"status\":\"inactive\"}}"),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
        return;
    }
    let (config, warning) = match storage::load_config() {
        Ok(c) => (c, None),
        Err(e) => (Config::default(), Some(e)),
    };
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            show_settings(app)
        }))
        .manage(Runtime {
            config: Mutex::new(config),
            beacon: Mutex::new(BeaconState::idle("starting", "Starting PromptBeacon…")),
            preview: Mutex::new(None),
            worker: Mutex::new(None),
            pending: Mutex::new((false, None)),
            permission: Mutex::new(false),
            settings_warning: Mutex::new(warning),
            diagnostics: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_status,
            get_detection_diagnostics,
            get_beacon,
            save_settings,
            request_accessibility,
            preview_beacon,
            stop_preview
        ])
        .setup(|app| {
            WebviewWindowBuilder::new(
                app,
                "beacon",
                WebviewUrl::App("index.html?overlay=1".into()),
            )
            .title("PromptBeacon indicator")
            .transparent(true)
            .decorations(false)
            .shadow(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .focusable(false)
            .visible(false)
            .inner_size(500., 400.)
            .build()?
            .set_ignore_cursor_events(true)?;
            use tauri::{
                menu::{Menu, MenuItem},
                tray::TrayIconBuilder,
            };
            let settings =
                MenuItem::with_id(app, "settings", "Open PromptBeacon", true, None::<&str>)?;
            let pause =
                MenuItem::with_id(app, "pause", "Pause / resume beacon", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit PromptBeacon", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&settings, &pause, &quit])?;
            let mut tray = TrayIconBuilder::with_id("promptbeacon")
                .tooltip("PromptBeacon")
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "settings" => show_settings(app),
                    "pause" => {
                        let state = app.state::<Runtime>();
                        let mut config = state.config.lock().unwrap_or_else(|e| e.into_inner());
                        config.enabled = !config.enabled;
                        if let Err(e) = storage::save_config(&config) {
                            *state
                                .settings_warning
                                .lock()
                                .unwrap_or_else(|e| e.into_inner()) = Some(e);
                        }
                        let _ = app.emit_to("main", "settings-changed", config.clone());
                    }
                    "quit" => app.exit(0),
                    _ => {}
                });
            if let Some(icon) = app.default_window_icon() {
                tray = tray.icon(icon.clone());
            }
            tray.build(app)?;
            let worker = monitor(app.handle().clone())?;
            *app.state::<Runtime>()
                .worker
                .lock()
                .unwrap_or_else(|e| e.into_inner()) = Some(worker);
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == "main" {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("PromptBeacon could not start")
        .run(|app, event| {
            if matches!(event, tauri::RunEvent::Exit) {
                if let Some(mut worker) = app
                    .state::<Runtime>()
                    .worker
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .take()
                {
                    worker.stop();
                }
            }
        });
}
