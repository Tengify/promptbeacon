use super::PlatformDetector;
use crate::{
    detection::{
        selection_control, supported_app, terminal_app, Control, Observation, ScanDiagnostics,
    },
    model::Bounds,
};
use std::{
    collections::VecDeque,
    ffi::{c_char, c_void, CString},
    ptr,
    time::{Duration, Instant},
};
type Ref = *const c_void;
#[repr(C)]
#[derive(Default)]
struct Point {
    x: f64,
    y: f64,
}
#[repr(C)]
#[derive(Default)]
struct Size {
    width: f64,
    height: f64,
}
#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    fn AXUIElementCreateSystemWide() -> Ref;
    fn AXUIElementCopyAttributeValue(element: Ref, attribute: Ref, value: *mut Ref) -> i32;
    fn AXUIElementSetAttributeValue(element: Ref, attribute: Ref, value: Ref) -> i32;
    fn AXUIElementGetPid(element: Ref, pid: *mut i32) -> i32;
    fn AXUIElementSetMessagingTimeout(element: Ref, timeout: f32) -> i32;
    fn AXIsProcessTrusted() -> bool;
    fn AXIsProcessTrustedWithOptions(options: Ref) -> bool;
    fn AXValueGetValue(value: Ref, kind: u32, out: *mut c_void) -> bool;
    fn AXValueGetTypeID() -> usize;
    static kAXTrustedCheckOptionPrompt: Ref;
}
#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFRelease(value: Ref);
    fn CFEqual(a: Ref, b: Ref) -> bool;
    fn CFRetain(value: Ref) -> Ref;
    fn CFStringCreateWithCString(allocator: Ref, value: *const c_char, encoding: u32) -> Ref;
    fn CFStringGetCString(value: Ref, buffer: *mut c_char, size: isize, encoding: u32) -> bool;
    fn CFGetTypeID(value: Ref) -> usize;
    fn CFStringGetTypeID() -> usize;
    fn CFArrayGetTypeID() -> usize;
    fn CFArrayGetCount(array: Ref) -> isize;
    fn CFArrayGetValueAtIndex(array: Ref, index: isize) -> Ref;
    fn CFDictionaryCreate(
        allocator: Ref,
        keys: *const Ref,
        values: *const Ref,
        count: isize,
        key_callbacks: Ref,
        value_callbacks: Ref,
    ) -> Ref;
    static kCFBooleanTrue: Ref;
}
#[link(name = "proc")]
unsafe extern "C" {
    fn proc_name(pid: i32, buffer: *mut c_void, size: u32) -> i32;
}
struct Owned(Ref);
impl Drop for Owned {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                CFRelease(self.0);
            }
        }
    }
}
fn string(value: &str) -> Owned {
    let s = CString::new(value).unwrap();
    Owned(unsafe { CFStringCreateWithCString(ptr::null(), s.as_ptr(), 0x08000100) })
}
fn attr(element: Ref, name: &str) -> Option<Owned> {
    let name = string(name);
    let mut result = ptr::null();
    let error = unsafe { AXUIElementCopyAttributeValue(element, name.0, &mut result) };
    let owned = Owned(result);
    if error == 0 && !owned.0.is_null() {
        Some(owned)
    } else {
        None
    }
}
fn text(element: Ref, name: &str) -> String {
    let Some(value) = attr(element, name) else {
        return String::new();
    };
    if unsafe { CFGetTypeID(value.0) != CFStringGetTypeID() } {
        return String::new();
    }
    let mut buf = [0u8; 768];
    if unsafe {
        CFStringGetCString(
            value.0,
            buf.as_mut_ptr().cast(),
            buf.len() as isize,
            0x08000100,
        )
    } {
        let end = buf.iter().position(|b| *b == 0).unwrap_or(buf.len());
        String::from_utf8_lossy(&buf[..end]).into()
    } else {
        String::new()
    }
}
fn rect(element: Ref) -> Option<Bounds> {
    let p = attr(element, "AXPosition")?;
    let s = attr(element, "AXSize")?;
    if unsafe { CFGetTypeID(p.0) != AXValueGetTypeID() || CFGetTypeID(s.0) != AXValueGetTypeID() } {
        return None;
    }
    let mut point = Point::default();
    let mut size = Size::default();
    if unsafe {
        AXValueGetValue(p.0, 1, (&mut point as *mut Point).cast())
            && AXValueGetValue(s.0, 2, (&mut size as *mut Size).cast())
    } {
        let b = Bounds {
            x: point.x,
            y: point.y,
            width: size.width,
            height: size.height,
            logical: true,
        };
        b.valid_rect().then_some(b)
    } else {
        None
    }
}
fn focused_composer(system: Ref, window: Ref, frame: &Bounds) -> Option<Owned> {
    let mut focused = attr(system, "AXFocusedUIElement")?;
    let owner = attr(focused.0, "AXWindow")?;
    if !unsafe { CFEqual(owner.0, window) } {
        return None;
    }
    let role = text(focused.0, "AXRole");
    if ![
        "AXTextArea",
        "AXTextField",
        "AXButton",
        "AXPopUpButton",
        "AXComboBox",
    ]
    .contains(&role.as_str())
    {
        return None;
    }
    let mut scope = None;
    for _ in 0..12 {
        if unsafe { CFEqual(focused.0, window) } {
            break;
        }
        if rect(focused.0).is_some_and(|r| r.composer_scope(frame)) {
            scope = Some(Owned(unsafe { CFRetain(focused.0) }));
        }
        let Some(parent) = attr(focused.0, "AXParent") else {
            break;
        };
        focused = parent;
    }
    scope
}
pub struct Detector {
    last_scan: Option<ScanDiagnostics>,
}
impl Detector {
    pub fn new() -> Self {
        Self { last_scan: None }
    }
}

impl PlatformDetector for Detector {
    fn diagnostics(&self) -> Option<ScanDiagnostics> {
        self.last_scan.clone()
    }
    fn permission(&self) -> bool {
        unsafe { AXIsProcessTrusted() }
    }
    fn request_permission(&self) {
        unsafe {
            let dict = Owned(CFDictionaryCreate(
                ptr::null(),
                &kAXTrustedCheckOptionPrompt,
                &kCFBooleanTrue,
                1,
                ptr::null(),
                ptr::null(),
            ));
            AXIsProcessTrustedWithOptions(dict.0);
        }
    }
    fn probe(&mut self) -> Result<Option<Observation>, String> {
        if !self.permission() {
            return Err("Allow PromptBeacon in System Settings → Privacy & Security → Accessibility, then return here.".into());
        }
        let system = Owned(unsafe { AXUIElementCreateSystemWide() });
        unsafe {
            AXUIElementSetMessagingTimeout(system.0, 0.04);
        }
        let Some(app) = attr(system.0, "AXFocusedApplication") else {
            return Ok(None);
        };
        let mut pid = 0;
        unsafe {
            AXUIElementGetPid(app.0, &mut pid);
        }
        let mut buffer = [0u8; 256];
        unsafe {
            proc_name(pid, buffer.as_mut_ptr().cast(), buffer.len() as u32);
        }
        let end = buffer.iter().position(|b| *b == 0).unwrap_or(buffer.len());
        let name = String::from_utf8_lossy(&buffer[..end]).to_string();
        if !supported_app(&name) && !terminal_app(&name) {
            return Ok(None);
        }
        unsafe {
            AXUIElementSetMessagingTimeout(app.0, 0.025);
        }
        let Some(window) = attr(app.0, "AXFocusedWindow") else {
            return Ok(None);
        };
        let title = text(window.0, "AXTitle");
        let frame = rect(window.0).filter(Bounds::valid);
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
        // Electron exposes its accessibility tree when this documented AX convention is enabled.
        let key = string("AXManualAccessibility");
        unsafe {
            AXUIElementSetAttributeValue(app.0, key.0, kCFBooleanTrue);
        }
        let focused_scope = frame
            .as_ref()
            .and_then(|f| focused_composer(system.0, window.0, f));
        let scope_name = if focused_scope.is_some() {
            "focused-composer"
        } else {
            "composer-region"
        };
        let started = Instant::now();
        let mut queue = VecDeque::from([(focused_scope.unwrap_or(window), 0usize)]);
        let mut controls = vec![];
        let mut visited = 0;
        let mut menu_open = false;
        let mut completed = true;
        while let Some((node, depth)) = queue.pop_front() {
            if visited >= 420 || started.elapsed() > Duration::from_millis(220) {
                completed = false;
                break;
            }
            visited += 1;
            let role = text(node.0, "AXRole");
            // Small control rectangles must never be validated as whole windows.
            let node_rect = rect(node.0);
            if scope_name == "composer-region"
                && depth > 0
                && role != "AXMenu"
                && role != "AXMenuItem"
                && frame
                    .as_ref()
                    .zip(node_rect.as_ref())
                    .is_some_and(|(w, n)| !n.in_composer_region(w))
            {
                continue;
            }
            if role == "AXMenu" || role == "AXMenuItem" {
                menu_open = true;
                continue;
            }
            if ["AXButton", "AXPopUpButton", "AXComboBox", "AXMenuButton"].contains(&role.as_str())
            {
                let label = text(node.0, "AXTitle");
                let description = text(node.0, "AXDescription");
                let identifier = text(node.0, "AXIdentifier").to_lowercase();
                let semantic = format!("{description} {identifier}").to_lowercase();
                let near_composer = frame
                    .as_ref()
                    .zip(node_rect.as_ref())
                    .is_some_and(|(w, n)| n.in_composer_region(w));
                if near_composer
                    || semantic.contains("model")
                    || semantic.contains("effort")
                    || semantic.contains("reasoning")
                {
                    let control = Control {
                        role: role.trim_start_matches("AX").into(),
                        label: if label.is_empty() { description } else { label },
                        value: text(node.0, "AXValue"),
                    };
                    // Only retain selection-like labels, never unrelated buttons/session titles.
                    if selection_control(&control) {
                        controls.push(control);
                    }
                }
            }
            if ["AXTextArea", "AXTextField", "AXStaticText"].contains(&role.as_str()) {
                continue;
            }
            if let Some(children) = attr(node.0, "AXChildren") {
                if unsafe { CFGetTypeID(children.0) == CFArrayGetTypeID() } {
                    let total = unsafe { CFArrayGetCount(children.0) };
                    if (depth >= 20 && total > 0)
                        || total > 160
                        || queue.len() + total as usize > 420
                    {
                        completed = false;
                        continue;
                    }
                    // Composer/action bars tend to be the final children. Visit them first.
                    for i in (0..total).rev() {
                        let child = unsafe { CFArrayGetValueAtIndex(children.0, i) };
                        if !child.is_null() {
                            queue.push_back((Owned(unsafe { CFRetain(child) }), depth + 1));
                        }
                    }
                }
            }
        }
        self.last_scan = Some(ScanDiagnostics {
            captured_at: crate::bridge::now_ms(),
            application: name.clone(),
            scope: scope_name.into(),
            visited_nodes: visited,
            elapsed_ms: started.elapsed().as_millis() as u64,
            completed,
            window_bounds_available: frame.is_some(),
            menu_open,
            matched_controls: controls.clone(),
        });
        if !completed {
            controls.clear();
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn core_foundation_ownership_stress() {
        for _ in 0..100_000 {
            let original = string("Dummy model selector");
            let retained = Owned(unsafe { CFRetain(original.0) });
            assert!(unsafe { CFEqual(original.0, retained.0) });
            drop(original);
            assert_eq!(unsafe { CFGetTypeID(retained.0) }, unsafe {
                CFStringGetTypeID()
            });
        }
    }
}
