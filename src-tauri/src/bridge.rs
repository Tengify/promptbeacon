use crate::{
    detection::effort_label,
    model::{BeaconState, Config},
    storage,
};
use serde::{Deserialize, Serialize};
#[cfg(unix)]
use std::io::Write;
use std::{
    io::Read,
    time::{SystemTime, UNIX_EPOCH},
};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeEvent {
    pub session_id: String,
    pub model: String,
    pub effort: Option<String>,
    pub updated_at: u64,
}
pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
pub fn token(session: &str) -> Option<String> {
    if !(8..=128).contains(&session.len())
        || !session
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return None;
    }
    Some(session.to_string())
}
pub fn read_title(title: &str, config: &Config) -> Option<BeaconState> {
    let index = title.find("PB:")?;
    let id = title[index + 3..]
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_')
        .next()?;
    token(id)?;
    let path = storage::data_dir()
        .ok()?
        .join("bridge")
        .join(format!("{id}.json"));
    let bytes = storage::read_limited(&path, 4096).ok()?;
    let event: BridgeEvent = serde_json::from_slice(&bytes).ok()?;
    if !fresh(&event, id, now_ms()) {
        return None;
    }
    Some(BeaconState {
        status: if event.effort.is_some() {
            "detected"
        } else {
            "partial"
        }
        .into(),
        application: Some("Claude Code terminal".into()),
        color: config.color_for(Some(&event.model)),
        model: Some(event.model),
        effort: event.effort,
        source: "claude-statusline".into(),
        message: "Live status line matched to this terminal title.".into(),
        bounds: None,
        preview: false,
    })
}
fn fresh(event: &BridgeEvent, id: &str, now: u64) -> bool {
    event.session_id == id
        && valid_model(&event.model)
        && event.effort.as_deref().is_none_or(|e| effort_label(e) == e)
        && now.saturating_sub(event.updated_at) <= 3500
        && event.updated_at <= now.saturating_add(1000)
}
fn valid_model(model: &str) -> bool {
    !model.trim().is_empty() && model.len() <= 120 && !model.chars().any(char::is_control)
}
/// Delete only expired bridge events with a valid session filename. Work is bounded.
pub fn prune_expired() {
    let Ok(dir) = storage::data_dir() else {
        return;
    };
    let Ok(entries) = std::fs::read_dir(dir.join("bridge")) else {
        return;
    };
    for entry in entries.take(512).flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "json") {
            continue;
        }
        if path
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(token)
            .is_none()
        {
            continue;
        }
        if entry
            .metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.elapsed().ok())
            .is_some_and(|age| age.as_secs() > 3600)
        {
            let _ = std::fs::remove_file(path);
        }
    }
}
pub fn run() -> Result<(), String> {
    let mut input = String::new();
    std::io::stdin()
        .take(65537)
        .read_to_string(&mut input)
        .map_err(|e| e.to_string())?;
    if input.len() > 65536 {
        return Err("Status-line input is too large.".into());
    }
    let value: serde_json::Value = serde_json::from_str(&input).map_err(|e| e.to_string())?;
    let session = value["session_id"]
        .as_str()
        .and_then(token)
        .ok_or("Missing or invalid session_id")?;
    let model = value["model"]["id"]
        .as_str()
        .or_else(|| value["model"]["display_name"].as_str())
        .ok_or("Missing model")?;
    if !valid_model(model) {
        return Err("Invalid model name".into());
    }
    let effort = value["effort"]["level"]
        .as_str()
        .map(effort_label)
        .filter(|v| v != "Unknown");
    let event = BridgeEvent {
        session_id: session.clone(),
        model: model.into(),
        effort: effort.clone(),
        updated_at: now_ms(),
    };
    let dir = storage::data_dir()?.join("bridge");
    storage::create_private_dir(&dir)?;
    storage::atomic_write(
        &dir.join(format!("{session}.json")),
        &serde_json::to_vec(&event).map_err(|e| e.to_string())?,
    )?;
    if std::env::args().any(|a| a == "--terminal-title") {
        set_terminal_title(&format!("Claude Code · PB:{session}"));
    }
    let config = storage::load_config().unwrap_or_default();
    let hex = config.color_for(Some(model));
    let rgb = (
        u8::from_str_radix(&hex[1..3], 16).unwrap_or(115),
        u8::from_str_radix(&hex[3..5], 16).unwrap_or(121),
        u8::from_str_radix(&hex[5..7], 16).unwrap_or(131),
    );
    println!(
        "\x1b[38;2;{};{};{}m{} · {}\x1b[0m",
        rgb.0,
        rgb.1,
        rgb.2,
        model,
        effort.unwrap_or_else(|| "Effort unavailable".into())
    );
    Ok(())
}
#[cfg(unix)]
fn set_terminal_title(title: &str) {
    if let Ok(mut tty) = std::fs::OpenOptions::new().write(true).open("/dev/tty") {
        let _ = write!(tty, "\x1b]2;{title}\x07");
    }
}
#[cfg(windows)]
fn set_terminal_title(title: &str) {
    use windows::{core::PCWSTR, Win32::System::Console::SetConsoleTitleW};
    let wide: Vec<u16> = title.encode_utf16().chain(Some(0)).collect();
    unsafe {
        let _ = SetConsoleTitleW(PCWSTR(wide.as_ptr()));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_path_traversal_and_title_controls() {
        assert!(token("../../settings").is_none());
        assert!(token("abcdefgh\x1b").is_none());
        assert!(token("abc12345-session").is_some());
    }
    #[test]
    fn stale_and_mismatched_sessions_are_never_shown() {
        let event = BridgeEvent {
            session_id: "session-123".into(),
            model: "claude-opus".into(),
            effort: Some("High".into()),
            updated_at: 10_000,
        };
        assert!(fresh(&event, "session-123", 10_200));
        assert!(!fresh(&event, "other-session", 10_200));
        assert!(!fresh(&event, "session-123", 13_501));
        assert!(!fresh(&event, "session-123", 8_000));
    }
    #[test]
    fn ordinary_title_cannot_claim_a_session() {
        assert!(read_title("Claude Code", &Config::default()).is_none());
    }
}
