use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ColorRule {
    pub name: String,
    pub pattern: String,
    pub color: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub enabled: bool,
    pub style: String,
    pub border_width: u32,
    pub tint_opacity: f64,
    pub badge_position: String,
    pub poll_ms: u64,
    pub rules: Vec<ColorRule>,
}
impl Default for Config {
    fn default() -> Self {
        let models = [
            ("GPT-6.1 Sol", "gpt-6.1-sol", "#2870D5"),
            ("GPT-6 Astra", "gpt-6-astra", "#8956C8"),
            ("GPT-6 Sol", "gpt-6-sol", "#168577"),
            ("GPT-6 Luna", "gpt-6-luna", "#AA691D"),
            ("Claude Opus", "opus", "#C45843"),
            ("Claude Sonnet", "sonnet", "#AF4589"),
            ("Claude Haiku", "haiku", "#537D24"),
        ];
        Self {
            enabled: true,
            style: "border".into(),
            border_width: 3,
            tint_opacity: 0.045,
            badge_position: "bottom-right".into(),
            poll_ms: 650,
            rules: models
                .into_iter()
                .map(|(name, pattern, color)| ColorRule {
                    name: name.into(),
                    pattern: pattern.into(),
                    color: color.into(),
                })
                .collect(),
        }
    }
}
impl Config {
    pub fn validate(&self) -> Result<(), String> {
        if !["border", "tint", "badge"].contains(&self.style.as_str()) {
            return Err("Choose a valid beacon style.".into());
        }
        if !["bottom-right", "bottom-left", "top-right"].contains(&self.badge_position.as_str()) {
            return Err("Choose a valid badge position.".into());
        }
        if !(1..=8).contains(&self.border_width)
            || !(0.0..=0.15).contains(&self.tint_opacity)
            || !self.tint_opacity.is_finite()
            || !(350..=3000).contains(&self.poll_ms)
        {
            return Err("Appearance or refresh value is out of range.".into());
        }
        if self.rules.len() > 64 {
            return Err("Use at most 64 model colors.".into());
        }
        let mut patterns = std::collections::HashSet::new();
        for rule in &self.rules {
            if rule.name.trim().is_empty()
                || rule.name.len() > 80
                || rule.pattern.trim().is_empty()
                || normalize(&rule.pattern).is_empty()
                || rule.pattern.len() > 80
            {
                return Err(
                    "Each model needs a name and a match phrase (up to 80 characters).".into(),
                );
            }
            if !patterns.insert(normalize(&rule.pattern)) {
                return Err("Each model match phrase must be unique.".into());
            }
            if rule.color.len() != 7
                || !rule.color.starts_with('#')
                || !rule.color[1..].bytes().all(|b| b.is_ascii_hexdigit())
            {
                return Err("Colors must use six-digit hex values, such as #2870D5.".into());
            }
        }
        Ok(())
    }
    pub fn color_for(&self, model: Option<&str>) -> String {
        let Some(model) = model else {
            return "#737983".into();
        };
        let model = normalize(model);
        self.rules
            .iter()
            .filter(|r| model.contains(&normalize(&r.pattern)))
            .max_by_key(|r| normalize(&r.pattern).len())
            .map(|r| r.color.clone())
            .unwrap_or_else(|| "#737983".into())
    }
}
pub fn normalize(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub logical: bool,
}
impl Bounds {
    /// Accessible controls can be only a few pixels high. Window sizing has separate limits.
    pub fn valid_rect(&self) -> bool {
        self.x.is_finite()
            && self.y.is_finite()
            && self.width.is_finite()
            && self.height.is_finite()
            && self.width > 0.0
            && self.height > 0.0
            && self.width <= 20000.0
            && self.height <= 20000.0
    }
    pub fn valid(&self) -> bool {
        self.valid_rect() && self.width >= 80.0 && self.height >= 60.0
    }
    pub fn in_composer_region(&self, window: &Bounds) -> bool {
        self.valid_rect()
            && self.y + self.height > window.y + window.height * 0.5
            && self.y < window.y + window.height
            && self.x + self.width > window.x
            && self.x < window.x + window.width
    }
    #[cfg(any(target_os = "macos", test))]
    pub fn composer_scope(&self, window: &Bounds) -> bool {
        self.in_composer_region(window)
            && self.width >= window.width * 0.30
            && self.height >= 48.0
            && self.height <= window.height * 0.55
            && self.y + self.height >= window.y + window.height - 80.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BeaconState {
    pub status: String,
    pub application: Option<String>,
    pub model: Option<String>,
    pub effort: Option<String>,
    pub source: String,
    pub message: String,
    pub color: String,
    pub bounds: Option<Bounds>,
    pub preview: bool,
}
impl BeaconState {
    pub fn idle(status: &str, message: impl Into<String>) -> Self {
        Self {
            status: status.into(),
            application: None,
            model: None,
            effort: None,
            source: "none".into(),
            message: message.into(),
            color: "#737983".into(),
            bounds: None,
            preview: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_defaults() {
        assert!(Config::default().validate().is_ok());
    }
    #[test]
    fn reject_css_injection() {
        let mut c = Config::default();
        c.rules[0].color = "red;display:none".into();
        assert!(c.validate().is_err());
    }
    #[test]
    fn longest_match_wins() {
        let c = Config::default();
        assert_eq!(c.color_for(Some("GPT 6.1 Sol")), "#2870D5");
        assert_eq!(c.color_for(None), "#737983");
    }
    #[test]
    fn reject_duplicate_patterns() {
        let mut c = Config::default();
        c.rules.push(c.rules[0].clone());
        assert!(c.validate().is_err());
    }
    #[test]
    fn compact_picker_buttons_are_valid_accessible_rectangles() {
        let window = Bounds {
            x: 100.,
            y: 80.,
            width: 900.,
            height: 740.,
            logical: true,
        };
        let model = Bounds {
            x: 380.,
            y: 740.,
            width: 130.,
            height: 28.,
            logical: true,
        };
        let effort = Bounds {
            x: 520.,
            y: 740.,
            width: 66.,
            height: 28.,
            logical: true,
        };
        assert!(model.valid_rect() && effort.valid_rect());
        assert!(model.in_composer_region(&window) && effort.in_composer_region(&window));
        assert!(!effort.valid()); // It is a control, not a usable overlay window.
    }
    #[test]
    fn offscreen_controls_are_not_in_the_composer_region() {
        let window = Bounds {
            x: 100.,
            y: 80.,
            width: 900.,
            height: 740.,
            logical: true,
        };
        let outside = Bounds {
            x: 380.,
            y: 900.,
            width: 130.,
            height: 28.,
            logical: true,
        };
        assert!(!outside.in_composer_region(&window));
    }
    #[test]
    fn focused_composer_scope_excludes_full_window_and_small_buttons() {
        let window = Bounds {
            x: 100.,
            y: 80.,
            width: 900.,
            height: 740.,
            logical: true,
        };
        let composer = Bounds {
            x: 360.,
            y: 650.,
            width: 610.,
            height: 150.,
            logical: true,
        };
        assert!(composer.composer_scope(&window));
        assert!(!window.composer_scope(&window));
        assert!(!Bounds {
            width: 66.,
            height: 28.,
            ..composer
        }
        .composer_scope(&window));
    }
    #[test]
    fn reject_bad_geometry() {
        assert!(!Bounds {
            x: f64::NAN,
            y: 0.,
            width: 500.,
            height: 400.,
            logical: true
        }
        .valid());
    }
}
