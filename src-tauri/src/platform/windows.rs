use super::PlatformDetector;
use crate::{
    detection::{selection_control, supported_app, terminal_app, Control, Observation},
    model::Bounds,
};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
use windows::{
    core::{Interface, PWSTR},
    Win32::{
        Foundation::CloseHandle,
        System::{
            Com::{
                CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER,
                COINIT_MULTITHREADED,
            },
            Threading::{
                OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
                PROCESS_QUERY_LIMITED_INFORMATION,
            },
        },
        UI::{
            Accessibility::{
                CUIAutomation8, IUIAutomation, IUIAutomation2, UIA_ButtonControlTypeId,
                UIA_ComboBoxControlTypeId, UIA_EditControlTypeId, UIA_MenuControlTypeId,
                UIA_MenuItemControlTypeId, UIA_TextControlTypeId,
            },
            WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId},
        },
    },
};
pub struct Detector {
    automation: Option<IUIAutomation>,
    initialized: bool,
}
impl Detector {
    pub fn new() -> Self {
        let initialized = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.is_ok();
        let automation: Option<IUIAutomation> =
            unsafe { CoCreateInstance(&CUIAutomation8, None, CLSCTX_INPROC_SERVER) }.ok();
        if let Some(api) = &automation {
            if let Ok(v2) = api.cast::<IUIAutomation2>() {
                unsafe {
                    let _ = v2.SetConnectionTimeout(100);
                    let _ = v2.SetTransactionTimeout(100);
                }
            }
        }
        Self {
            automation,
            initialized,
        }
    }
}
impl Drop for Detector {
    fn drop(&mut self) {
        self.automation.take();
        if self.initialized {
            unsafe {
                CoUninitialize();
            }
        }
    }
}
impl PlatformDetector for Detector {
    fn probe(&mut self) -> Result<Option<Observation>, String> {
        unsafe {
            let automation = self
                .automation
                .as_ref()
                .ok_or("Windows UI Automation could not start.")?;
            let hwnd = GetForegroundWindow();
            if hwnd.0.is_null() {
                return Ok(None);
            }
            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid)
                .map_err(|e| e.to_string())?;
            let mut path = [0u16; 1024];
            let mut length = path.len() as u32;
            let result = QueryFullProcessImageNameW(
                process,
                PROCESS_NAME_WIN32,
                PWSTR(path.as_mut_ptr()),
                &mut length,
            );
            let _ = CloseHandle(process);
            result.map_err(|e| e.to_string())?;
            let path = String::from_utf16_lossy(&path[..length as usize]);
            let name = path.rsplit(['\\', '/']).next().unwrap_or(&path).to_string();
            if !supported_app(&name) && !terminal_app(&name) {
                return Ok(None);
            }
            let mut title_buf = [0u16; 512];
            let len = GetWindowTextW(hwnd, &mut title_buf);
            let title = String::from_utf16_lossy(&title_buf[..len.max(0) as usize]);
            let root = automation
                .ElementFromHandle(hwnd)
                .map_err(|e| e.to_string())?;
            let r = root.CurrentBoundingRectangle().map_err(|e| e.to_string())?;
            let b = Bounds {
                x: r.left as f64,
                y: r.top as f64,
                width: (r.right - r.left) as f64,
                height: (r.bottom - r.top) as f64,
                logical: false,
            };
            let frame = b.valid().then_some(b);
            if terminal_app(&name) {
                return Ok(Some(Observation {
                    application: name,
                    title,
                    bounds: frame,
                    controls: vec![],
                    menu_open: false,
                    terminal: true,
                }));
            }
            let walker = automation.ControlViewWalker().map_err(|e| e.to_string())?;
            let mut queue = VecDeque::from([(root, 0usize)]);
            let mut controls = vec![];
            let mut visited = 0;
            let mut menu_open = false;
            let started = Instant::now();
            while let Some((node, depth)) = queue.pop_front() {
                if visited >= 420 || started.elapsed() > Duration::from_millis(220) {
                    controls.clear();
                    break;
                }
                visited += 1;
                if node.CurrentIsOffscreen().is_ok_and(|v| v.as_bool()) {
                    continue;
                }
                let kind = node.CurrentControlType().unwrap_or_default();
                if kind == UIA_MenuControlTypeId || kind == UIA_MenuItemControlTypeId {
                    menu_open = true;
                    continue;
                }
                if kind == UIA_ButtonControlTypeId || kind == UIA_ComboBoxControlTypeId {
                    let label = node
                        .CurrentName()
                        .map(|v| v.to_string())
                        .unwrap_or_default();
                    let id = node
                        .CurrentAutomationId()
                        .map(|v| v.to_string().to_lowercase())
                        .unwrap_or_default();
                    let help = node
                        .CurrentHelpText()
                        .map(|v| v.to_string().to_lowercase())
                        .unwrap_or_default();
                    let near = node
                        .CurrentBoundingRectangle()
                        .ok()
                        .zip(frame.as_ref())
                        .is_some_and(|(n, w)| {
                            Bounds {
                                x: n.left as f64,
                                y: n.top as f64,
                                width: (n.right - n.left) as f64,
                                height: (n.bottom - n.top) as f64,
                                logical: false,
                            }
                            .in_composer_region(w)
                        });
                    if near
                        || id.contains("model")
                        || id.contains("effort")
                        || help.contains("model")
                        || help.contains("effort")
                    {
                        let control = Control {
                            role: if kind == UIA_ButtonControlTypeId {
                                "button"
                            } else {
                                "combobox"
                            }
                            .into(),
                            label,
                            value: String::new(),
                        };
                        if selection_control(&control) {
                            controls.push(control);
                        }
                    }
                }
                if kind == UIA_EditControlTypeId || kind == UIA_TextControlTypeId {
                    continue;
                }
                if let Ok(mut child) = walker.GetFirstChildElement(&node) {
                    if depth >= 14 {
                        controls.clear();
                        break;
                    }
                    for i in 0..160 {
                        if queue.len() >= 420 {
                            controls.clear();
                            queue.clear();
                            return Ok(Some(Observation {
                                application: name,
                                title,
                                bounds: frame,
                                controls,
                                menu_open,
                                terminal: false,
                            }));
                        }
                        queue.push_back((child.clone(), depth + 1));
                        match walker.GetNextSiblingElement(&child) {
                            Ok(next) if i < 159 => child = next,
                            Ok(_) => {
                                controls.clear();
                                queue.clear();
                                return Ok(Some(Observation {
                                    application: name,
                                    title,
                                    bounds: frame,
                                    controls,
                                    menu_open,
                                    terminal: false,
                                }));
                            }
                            Err(_) => break,
                        }
                    }
                }
            }
            Ok(Some(Observation {
                application: name,
                title,
                bounds: frame,
                controls,
                menu_open,
                terminal: false,
            }))
        }
    }
}
