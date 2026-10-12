use crate::model::{BeaconState, Bounds, Config};
use regex::Regex;
use std::{collections::BTreeSet, sync::LazyLock};

#[derive(Clone, Debug, serde::Serialize)]
pub struct Control {
    pub role: String,
    pub label: String,
    pub value: String,
}
#[derive(Clone, Debug)]
pub struct Observation {
    pub application: String,
    pub title: String,
    pub bounds: Option<Bounds>,
    pub controls: Vec<Control>,
    pub menu_open: bool,
    pub terminal: bool,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanDiagnostics {
    pub captured_at: u64,
    pub application: String,
    pub scope: String,
    pub visited_nodes: usize,
    pub elapsed_ms: u64,
    pub completed: bool,
    pub window_bounds_available: bool,
    pub menu_open: bool,
    pub matched_controls: Vec<Control>,
}
impl ScanDiagnostics {
    pub fn stable_key(&self) -> String {
        serde_json::json!({"application":self.application,"scope":self.scope,"completed":self.completed,
            "bounds":self.window_bounds_available,"menu":self.menu_open,"controls":self.matched_controls}).to_string()
    }
}
fn selected_model_text(text: &str) -> Option<regex::Match<'_>> {
    if text.len() > 180 {
        return None;
    }
    let found = MODEL.find(text)?;
    let prefix = crate::model::normalize(&text[..found.start()]);
    if ![
        "",
        "model",
        "selectmodel",
        "choosemodel",
        "changemodel",
        "modelselector",
        "modelselection",
        "currentmodel",
        "selectedmodel",
    ]
    .contains(&prefix.as_str())
    {
        return None;
    }
    let remainder = EFFORT.replace_all(&text[found.end()..], "");
    let suffix = crate::model::normalize(&remainder);
    if ![
        "",
        "model",
        "selected",
        "modelselected",
        "reasoningeffort",
        "reasoning",
        "effort",
        "fast",
        "fastmode",
        "modelselector",
        "modelselection",
    ]
    .contains(&suffix.as_str())
    {
        return None;
    }
    Some(found)
}
fn selected_effort_text(text: &str) -> Option<&str> {
    if text.len() > 180 {
        return None;
    }
    let found = EFFORT.find(text)?;
    let prefix = crate::model::normalize(&text[..found.start()]);
    let suffix = crate::model::normalize(&text[found.end()..]);
    if [
        "",
        "effort",
        "reasoning",
        "reasoningeffort",
        "selectedeffort",
    ]
    .contains(&prefix.as_str())
        && ["", "selected"].contains(&suffix.as_str())
    {
        Some(found.as_str())
    } else {
        None
    }
}
pub fn selection_control(control: &Control) -> bool {
    for text in [&control.label, &control.value] {
        if text.len() > 180 {
            continue;
        }
        if selected_model_text(text).is_some() {
            return true;
        }
        if selected_effort_text(text).is_some() {
            return true;
        }
    }
    false
}

pub fn supported_app(name: &str) -> bool {
    let n = name.to_lowercase();
    n.contains("codex")
        || n == "chatgpt"
        || n == "chatgpt.exe"
        || n == "claude"
        || n == "claude.exe"
}
pub fn terminal_app(name: &str) -> bool {
    let n = name.to_lowercase();
    [
        "iterm",
        "terminal",
        "wezterm",
        "alacritty",
        "kitty",
        "konsole",
        "gnome-terminal",
        "ghostty",
        "xterm",
        "tilix",
    ]
    .iter()
    .any(|s| n.contains(s))
}
static MODEL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:gpt[ -]?\d(?:\.\d+)*(?:[ -](?:sol|astra|luna|codex|mini|nano|pro|turbo))*(?:[ -]\d{4}[ -]\d{2}[ -]\d{2})?|o[1-9](?:[ -](?:mini|pro))?|(?:claude[ -])?(?:opus|sonnet|haiku)(?:[ -]\d(?:[.-]\d+)*)?)\b").unwrap()
});
static EFFORT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(extra[ -]?high|xhigh|maximum|max|medium|high|low|minimal|none|ultra)\b")
        .unwrap()
});

pub fn selected(controls: &[Control], menu_open: bool) -> (Option<String>, Option<String>) {
    if menu_open {
        return (None, None);
    }
    let mut models = std::collections::BTreeMap::new();
    let mut efforts = BTreeSet::new();
    for control in controls {
        let role = control.role.to_lowercase();
        if ![
            "button",
            "popupbutton",
            "combobox",
            "menubutton",
            "togglebutton",
        ]
        .contains(&role.as_str())
        {
            continue;
        }
        if !selection_control(control) {
            continue;
        }
        for text in [&control.label, &control.value] {
            if text.len() > 180 {
                continue;
            }
            if let Some(found) = selected_model_text(text) {
                let value = found.as_str().trim().to_string();
                models.insert(crate::model::normalize(&value), value.to_lowercase());
                for e in EFFORT.find_iter(&text[found.end()..]) {
                    efforts.insert(effort_label(e.as_str()));
                }
            } else {
                if let Some(effort) = selected_effort_text(text) {
                    efforts.insert(effort_label(effort));
                }
            }
        }
    }
    (
        if models.len() == 1 {
            models.into_values().next().map(|m| display_model(&m))
        } else {
            None
        },
        if efforts.len() == 1 {
            efforts.into_iter().next()
        } else {
            None
        },
    )
}
fn display_model(model: &str) -> String {
    model
        .split_whitespace()
        .map(|word| {
            if word.starts_with("gpt") {
                word.to_uppercase()
            } else {
                let mut c = word.chars();
                c.next()
                    .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
                    .unwrap_or_default()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
pub fn effort_label(value: &str) -> String {
    match value.to_lowercase().replace([' ', '-'], "").as_str() {
        "xhigh" | "extrahigh" => "Extra High",
        "max" | "maximum" => "Max",
        "low" => "Low",
        "medium" => "Medium",
        "high" => "High",
        "minimal" => "Minimal",
        "none" => "None",
        "ultra" => "Ultra",
        _ => "Unknown",
    }
    .into()
}
pub fn from_observation(observation: Observation, config: &Config) -> BeaconState {
    let (model, effort) = selected(&observation.controls, observation.menu_open);
    let status = if model.is_some() && effort.is_some() {
        "detected"
    } else if model.is_some() {
        "partial"
    } else {
        "unknown"
    };
    let message = if observation.menu_open {
        "Selection menu open. Choose a model to refresh the beacon."
    } else if model.is_none() {
        "Selected model is not exposed by this app. The beacon is neutral."
    } else if effort.is_none() {
        "Model detected. Effort is not exposed or is unsupported."
    } else {
        "Current selection read from the active window."
    };
    BeaconState {
        status: status.into(),
        application: Some(observation.application),
        color: config.color_for(model.as_deref()),
        model,
        effort,
        source: "accessibility".into(),
        message: message.into(),
        bounds: observation.bounds,
        preview: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn control(role: &str, label: &str) -> Control {
        Control {
            role: role.into(),
            label: label.into(),
            value: String::new(),
        }
    }
    #[test]
    fn reads_combined_selection() {
        let (m, e) = selected(&[control("button", "GPT-6.1 Sol Extra High")], false);
        assert_eq!(m.as_deref(), Some("GPT-6.1 Sol"));
        assert_eq!(e.as_deref(), Some("Extra High"));
    }
    #[test]
    fn ignores_prompt_and_transcript() {
        let (m, e) = selected(
            &[
                control("text", "GPT-6.1 Sol Extra High"),
                control("textbox", "Claude Opus high"),
            ],
            false,
        );
        assert!(m.is_none() && e.is_none());
    }
    #[test]
    fn open_menu_is_unknown() {
        assert_eq!(
            selected(&[control("button", "GPT-6.1 Sol")], true),
            (None, None)
        );
    }
    #[test]
    fn multiple_panes_are_ambiguous() {
        let (m, _) = selected(
            &[
                control("button", "GPT-6.1 Sol"),
                control("button", "GPT-6 Astra"),
            ],
            false,
        );
        assert!(m.is_none());
    }
    #[test]
    fn separate_effort() {
        let (m, e) = selected(
            &[
                control("combobox", "Claude Sonnet 4.6"),
                control("button", "Reasoning effort High"),
            ],
            false,
        );
        assert!(m.is_some());
        assert_eq!(e.as_deref(), Some("High"));
    }
    #[test]
    fn claude_labeled_effort_picker_is_retained_and_read() {
        let effort = control("popupbutton", "Effort: Medium");
        assert!(selection_control(&effort));
        let (model, level) = selected(
            &[control("popupbutton", "Model: Claude Sonnet"), effort],
            false,
        );
        assert!(model.is_some());
        assert_eq!(level.as_deref(), Some("Medium"));
    }
    #[test]
    fn effort_labels_do_not_capture_unrelated_buttons_or_conflicting_levels() {
        assert!(!selection_control(&control(
            "button",
            "My Medium effort project"
        )));
        let (_, effort) = selected(
            &[
                control("popupbutton", "Effort: Low"),
                control("popupbutton", "Effort: High"),
            ],
            false,
        );
        assert!(effort.is_none());
    }
    #[test]
    fn unrelated_high_button_does_not_parse_partial_word() {
        assert!(selected(&[control("button", "Highlight")], false)
            .1
            .is_none());
    }
    #[test]
    fn transcript_title_is_not_a_selection() {
        assert!(
            selected(&[control("button", "Explain GPT-6.1 Sol to me")], false)
                .0
                .is_none()
        );
    }
    #[test]
    fn duplicate_label_and_value_are_one_model() {
        let c = Control {
            role: "popupbutton".into(),
            label: "GPT-6.1 Sol".into(),
            value: "GPT 6.1 Sol".into(),
        };
        assert!(selected(&[c], false).0.is_some());
    }
    #[test]
    fn model_named_conversation_buttons_are_not_selected_models() {
        let (model, _) = selected(
            &[control(
                "button",
                "GPT-6.1 Sol for my private client project",
            )],
            false,
        );
        assert!(model.is_none());
    }
    #[test]
    fn labeled_picker_with_parenthesized_effort_is_detected() {
        let (model, effort) = selected(
            &[control(
                "popupbutton",
                "Selected model: GPT-6.1 Sol (Extra High)",
            )],
            false,
        );
        assert_eq!(model.as_deref(), Some("GPT-6.1 Sol"));
        assert_eq!(effort.as_deref(), Some("Extra High"));
    }
    #[test]
    fn stale_selection_is_not_carried_over() {
        let c = Config::default();
        let obs = Observation {
            application: "Codex".into(),
            title: String::new(),
            bounds: None,
            controls: vec![],
            menu_open: false,
            terminal: false,
        };
        assert_eq!(from_observation(obs, &c).model, None);
    }
}
