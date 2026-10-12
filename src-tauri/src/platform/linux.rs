use super::PlatformDetector;
use crate::{
    detection::{selection_control, supported_app, terminal_app, Control, Observation},
    model::Bounds,
};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
use x11rb::{
    connection::Connection as XConnection,
    protocol::xproto::{AtomEnum, ConnectionExt},
};
use zbus::{
    blocking::{Connection, Proxy},
    zvariant::OwnedObjectPath,
};
type Object = (String, OwnedObjectPath);
pub struct Detector {
    bus: Option<Connection>,
    x11: Option<(x11rb::rust_connection::RustConnection, usize)>,
}
impl Detector {
    pub fn new() -> Self {
        Self {
            bus: None,
            x11: None,
        }
    }
    fn bus(&mut self) -> Result<&Connection, String> {
        if self.bus.is_none() {
            let session = zbus::blocking::connection::Builder::session()
                .map_err(|e| e.to_string())?
                .method_timeout(Duration::from_millis(100))
                .build()
                .map_err(|e| e.to_string())?;
            let proxy = Proxy::new(&session, "org.a11y.Bus", "/org/a11y/bus", "org.a11y.Bus")
                .map_err(|e| e.to_string())?;
            let address: String = proxy
                .call("GetAddress", &())
                .map_err(|e| format!("Enable desktop accessibility (AT-SPI): {e}"))?;
            self.bus = Some(
                zbus::blocking::connection::Builder::address(address.as_str())
                    .map_err(|e| e.to_string())?
                    .method_timeout(Duration::from_millis(50))
                    .build()
                    .map_err(|e| e.to_string())?,
            );
        }
        self.bus
            .as_ref()
            .ok_or_else(|| "AT-SPI is unavailable.".into())
    }
}
fn accessible<'a>(bus: &'a Connection, obj: &'a Object) -> zbus::Result<Proxy<'a>> {
    Proxy::new(
        bus,
        obj.0.as_str(),
        obj.1.as_str(),
        "org.a11y.atspi.Accessible",
    )
}
fn extents(bus: &Connection, obj: &Object) -> Option<Bounds> {
    let p = Proxy::new(
        bus,
        obj.0.as_str(),
        obj.1.as_str(),
        "org.a11y.atspi.Component",
    )
    .ok()?;
    let (x, y, w, h): (i32, i32, i32, i32) = p.call("GetExtents", &(0u32,)).ok()?;
    let b = Bounds {
        x: x as f64,
        y: y as f64,
        width: w as f64,
        height: h as f64,
        logical: false,
    };
    b.valid_rect().then_some(b)
}
fn x11_focus(
    c: &x11rb::rust_connection::RustConnection,
    screen: usize,
) -> Option<(String, String, Bounds)> {
    let root = c.setup().roots[screen].root;
    let atom = |name: &[u8]| {
        c.intern_atom(false, name)
            .ok()?
            .reply()
            .ok()
            .map(|r| r.atom)
    };
    let active = atom(b"_NET_ACTIVE_WINDOW")?;
    let win = c
        .get_property(false, root, active, AtomEnum::WINDOW, 0, 1)
        .ok()?
        .reply()
        .ok()?
        .value32()?
        .next()?;
    let pid = c
        .get_property(false, win, atom(b"_NET_WM_PID")?, AtomEnum::CARDINAL, 0, 1)
        .ok()?
        .reply()
        .ok()?
        .value32()?
        .next()?;
    let name = std::fs::read_to_string(format!("/proc/{pid}/comm"))
        .ok()?
        .trim()
        .to_string();
    let title = c
        .get_property(
            false,
            win,
            atom(b"_NET_WM_NAME")?,
            atom(b"UTF8_STRING")?,
            0,
            512,
        )
        .ok()?
        .reply()
        .ok()
        .map(|r| String::from_utf8_lossy(&r.value).to_string())
        .unwrap_or_default();
    let g = c.get_geometry(win).ok()?.reply().ok()?;
    let t = c
        .translate_coordinates(win, root, 0, 0)
        .ok()?
        .reply()
        .ok()?;
    Some((
        name,
        title,
        Bounds {
            x: t.dst_x as f64,
            y: t.dst_y as f64,
            width: g.width as f64,
            height: g.height as f64,
            logical: false,
        },
    ))
}
impl PlatformDetector for Detector {
    fn probe(&mut self) -> Result<Option<Observation>, String> {
        let wayland = std::env::var_os("WAYLAND_DISPLAY").is_some();
        let focused = if wayland {
            None
        } else {
            if self.x11.is_none() {
                self.x11 = x11rb::connect(None).ok();
            }
            let focus = self
                .x11
                .as_ref()
                .and_then(|(c, screen)| x11_focus(c, *screen));
            if focus.is_none() {
                self.x11 = None;
            }
            focus
        };
        if let Some((name, title, b)) = &focused {
            if terminal_app(name) {
                return Ok(Some(Observation {
                    application: name.clone(),
                    title: title.clone(),
                    bounds: Some(b.clone()),
                    controls: vec![],
                    menu_open: false,
                    terminal: true,
                }));
            }
            if !supported_app(name) {
                return Ok(None);
            }
        }
        let bus = self.bus()?;
        let registry: Object = (
            "org.a11y.atspi.Registry".into(),
            OwnedObjectPath::try_from("/org/a11y/atspi/accessible/root").unwrap(),
        );
        let apps_result: Result<Vec<Object>, _> =
            accessible(bus, &registry).and_then(|desktop| desktop.call("GetChildren", &()));
        let apps = match apps_result {
            Ok(apps) => apps,
            Err(error) => {
                let message = error.to_string();
                self.bus = None;
                return Err(message);
            }
        };
        let bus = self.bus.as_ref().ok_or("AT-SPI is unavailable.")?;
        let started = Instant::now();
        let mut candidates = vec![];
        for app in apps.into_iter().take(80) {
            if started.elapsed() > Duration::from_millis(220) {
                return Ok(None);
            }
            let p = match accessible(bus, &app) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let name: String = p.get_property("Name").unwrap_or_default();
            if !supported_app(&name) && !terminal_app(&name) {
                continue;
            }
            let windows: Vec<Object> = p.call("GetChildren", &()).unwrap_or_default();
            for window in windows.into_iter().take(12) {
                let wp = match accessible(bus, &window) {
                    Ok(p) => p,
                    Err(_) => continue,
                };
                let title: String = wp.get_property("Name").unwrap_or_default();
                let states: Vec<u32> = wp.call("GetState", &()).unwrap_or_default();
                let active = states.first().is_some_and(|s| s & (1 << 1) != 0);
                let matches = focused.as_ref().is_some_and(|(focused_name, t, _)| {
                    !t.is_empty()
                        && title == *t
                        && crate::model::normalize(focused_name) == crate::model::normalize(&name)
                });
                drop(wp);
                if matches || (focused.is_none() && active) {
                    candidates.push((name.clone(), title, window));
                }
            }
        }
        if candidates.len() != 1 {
            return Ok(None);
        }
        let (name, title, root) = candidates.remove(0);
        let frame = if wayland {
            None
        } else {
            focused
                .map(|(_, _, b)| b)
                .or_else(|| extents(bus, &root).filter(Bounds::valid))
        };
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
        let mut queue = VecDeque::from([(root, 0usize)]);
        let mut controls = vec![];
        let mut menu_open = false;
        let mut visited = 0;
        let scan = Instant::now();
        while let Some((obj, depth)) = queue.pop_front() {
            if visited >= 420 || scan.elapsed() > Duration::from_millis(220) {
                controls.clear();
                break;
            }
            visited += 1;
            let p = match accessible(bus, &obj) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let role: String = p.call("GetRoleName", &()).unwrap_or_default();
            if role == "menu" || role == "menu item" {
                menu_open = true;
                continue;
            }
            if ["push button", "combo box", "toggle button", "menu button"].contains(&role.as_str())
            {
                let label: String = p.get_property("Name").unwrap_or_default();
                let description: String = p.get_property("Description").unwrap_or_default();
                let semantic = description.to_lowercase();
                let near = match (&frame, extents(bus, &obj)) {
                    (Some(w), Some(n)) => n.in_composer_region(w),
                    (None, _) => true,
                    _ => false,
                };
                if near || semantic.contains("model") || semantic.contains("effort") {
                    let control = Control {
                        role: match role.as_str() {
                            "combo box" => "combobox",
                            "toggle button" => "togglebutton",
                            "menu button" => "menubutton",
                            _ => "button",
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
            if ["text", "paragraph", "entry"].contains(&role.as_str()) {
                continue;
            }
            let children: Vec<Object> = p.call("GetChildren", &()).unwrap_or_default();
            if (!children.is_empty() && depth >= 14)
                || children.len() > 160
                || queue.len() + children.len() > 420
            {
                controls.clear();
                break;
            }
            for child in children {
                queue.push_back((child, depth + 1));
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
