use crate::model::Config;
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub fn data_dir() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("PROMPTBEACON_DATA_DIR") {
        return Ok(PathBuf::from(path));
    }
    dirs::data_local_dir()
        .map(|p| p.join("PromptBeacon"))
        .ok_or_else(|| "Cannot locate your local application data folder.".into())
}
/// Bound allocation before deserialization, even if a local file is corrupt or replaced.
pub fn read_limited(path: &Path, limit: usize) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    fs::File::open(path)
        .map_err(|e| e.to_string())?
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > limit {
        return Err("Local data exceeds its size limit.".into());
    }
    Ok(bytes)
}
pub fn create_private_dir(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(|e| e.to_string())?;
    }
    Ok(())
}
pub fn load_config() -> Result<Config, String> {
    let path = data_dir()?.join("settings.json");
    if !path.exists() {
        return Ok(Config::default());
    }
    let c: Config = serde_json::from_slice(&read_limited(&path, 65536)?)
        .map_err(|e| format!("Saved settings could not be read: {e}"))?;
    c.validate()?;
    Ok(c)
}
pub fn save_config(config: &Config) -> Result<(), String> {
    config.validate()?;
    let dir = data_dir()?;
    create_private_dir(&dir)?;
    atomic_write(
        &dir.join("settings.json"),
        &serde_json::to_vec_pretty(config).map_err(|e| e.to_string())?,
    )
}
/// Unique, private, exclusively created temporary files; replace without a delete gap.
pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path.parent().ok_or("Local data path has no parent.")?;
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    temp.write_all(bytes).map_err(|e| e.to_string())?;
    temp.as_file().sync_all().map_err(|e| e.to_string())?;
    temp.persist(path).map_err(|e| e.error.to_string())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oversized_files_are_rejected_before_parsing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("dummy.json");
        fs::write(&path, vec![b'x'; 4097]).unwrap();
        assert!(read_limited(&path, 4096).is_err());
    }
    #[test]
    fn concurrent_replacements_are_complete_and_leave_no_temp_files() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("dummy.json");
        std::thread::scope(|s| {
            for i in 0..8u8 {
                let path = &path;
                s.spawn(move || {
                    for _ in 0..25 {
                        atomic_write(path, &vec![i; 2048]).unwrap();
                    }
                });
            }
        });
        let bytes = fs::read(&path).unwrap();
        assert_eq!(bytes.len(), 2048);
        assert!(bytes.iter().all(|v| *v == bytes[0]));
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }
}
