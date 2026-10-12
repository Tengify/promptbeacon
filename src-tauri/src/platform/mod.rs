use crate::detection::Observation;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
pub use linux::Detector;
#[cfg(target_os = "macos")]
pub use macos::Detector;
#[cfg(target_os = "windows")]
pub use windows::Detector;
pub trait PlatformDetector {
    fn probe(&mut self) -> Result<Option<Observation>, String>;
    fn permission(&self) -> bool {
        true
    }
    fn request_permission(&self) {}
    fn diagnostics(&self) -> Option<crate::detection::ScanDiagnostics> {
        None
    }
}
