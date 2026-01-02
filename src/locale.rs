//! Locale configuration for Material UI.
//!
//! This is intentionally lightweight (no ICU dependency). The goal is to:
//! - Let apps set a locale early via a resource (similar to `MaterialTheme`).
//! - Allow per-component overrides via a component.
//! - Provide a few locale-driven defaults (currently: date input pattern).

use bevy::prelude::*;

/// Material locale resource.
///
/// This is a BCP-47-ish tag (e.g. `"en-US"`, `"en-GB"`, `"fr-FR"`).
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub struct MaterialLocale {
    pub tag: String,
}

impl Default for MaterialLocale {
    fn default() -> Self {
        Self {
            tag: system_locale_tag().unwrap_or_else(|| "en-US".to_string()),
        }
    }
}

impl MaterialLocale {
    pub fn new(tag: impl Into<String>) -> Self {
        Self { tag: tag.into() }
    }
}

/// Per-component locale override.
///
/// Attach this to a component root (e.g. a date picker root entity) to override
/// the locale-driven defaults for that component.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct MaterialLocaleOverride {
    pub tag: String,
}

impl MaterialLocaleOverride {
    pub fn new(tag: impl Into<String>) -> Self {
        Self { tag: tag.into() }
    }
}

/// Date input field order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateFieldOrder {
    /// Month / Day / Year
    Mdy,
    /// Day / Month / Year
    Dmy,
    /// Year / Month / Day
    Ymd,
}

/// Date input pattern for text fields.
///
/// Notes:
/// - This is focused on numeric date entry and delimiter insertion.
/// - The formatted length is always 10 (2/2/4 or 4/2/2 + 2 separators).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateInputPattern {
    pub order: DateFieldOrder,
    pub separator: char,
}

impl DateInputPattern {
    pub const fn new(order: DateFieldOrder, separator: char) -> Self {
        Self { order, separator }
    }

    pub const fn formatted_len(self) -> usize {
        // 2 + 1 + 2 + 1 + 4, or 4 + 1 + 2 + 1 + 2
        10
    }

    pub fn hint(self) -> String {
        match self.order {
            DateFieldOrder::Mdy => format!("MM{}DD{}YYYY", self.separator, self.separator),
            DateFieldOrder::Dmy => format!("DD{}MM{}YYYY", self.separator, self.separator),
            DateFieldOrder::Ymd => format!("YYYY{}MM{}DD", self.separator, self.separator),
        }
    }

    pub fn normalize_digits(self, input: &str) -> String {
        let digits: String = input
            .chars()
            .filter(|c| c.is_ascii_digit())
            .take(8)
            .collect();
        let len = digits.len();

        match self.order {
            DateFieldOrder::Mdy | DateFieldOrder::Dmy => {
                if len <= 2 {
                    return digits;
                }
                if len <= 4 {
                    return format!("{}{}{}", &digits[0..2], self.separator, &digits[2..len]);
                }

                format!(
                    "{}{}{}{}{}",
                    &digits[0..2],
                    self.separator,
                    &digits[2..4],
                    self.separator,
                    &digits[4..len]
                )
            }
            DateFieldOrder::Ymd => {
                if len <= 4 {
                    return digits;
                }
                if len <= 6 {
                    return format!("{}{}{}", &digits[0..4], self.separator, &digits[4..len]);
                }

                format!(
                    "{}{}{}{}{}",
                    &digits[0..4],
                    self.separator,
                    &digits[4..6],
                    self.separator,
                    &digits[6..len]
                )
            }
        }
    }

    pub fn try_parse_complete(self, input: &str) -> Option<(i32, u8, u8)> {
        if input.len() != self.formatted_len() {
            return None;
        }

        let parts: Vec<&str> = input.split(self.separator).collect();
        if parts.len() != 3 {
            return None;
        }

        let (year_str, month_str, day_str) = match self.order {
            DateFieldOrder::Mdy => (parts[2], parts[0], parts[1]),
            DateFieldOrder::Dmy => (parts[2], parts[1], parts[0]),
            DateFieldOrder::Ymd => (parts[0], parts[1], parts[2]),
        };

        let month: u8 = month_str.parse().ok()?;
        let day: u8 = day_str.parse().ok()?;
        let year: i32 = year_str.parse().ok()?;

        Some((year, month, day))
    }
}

/// Resolve a reasonable date input pattern for a locale tag.
///
/// This is a pragmatic heuristic (not full CLDR):
/// - `*-US` / `*-PH` / `*-BZ` => MDY
/// - `zh*` / `ja*` / `ko*` => YMD
/// - otherwise => DMY
pub fn date_input_pattern_for_locale(tag: &str) -> DateInputPattern {
    let normalized = tag.trim().replace('_', "-");
    let mut parts = normalized.split('-');

    let language = parts.next().unwrap_or("").to_ascii_lowercase();
    let region = parts.next().unwrap_or("").to_ascii_uppercase();

    if matches!(region.as_str(), "US" | "PH" | "BZ") {
        return DateInputPattern::new(DateFieldOrder::Mdy, '/');
    }

    if matches!(language.as_str(), "zh" | "ja" | "ko") {
        return DateInputPattern::new(DateFieldOrder::Ymd, '/');
    }

    DateInputPattern::new(DateFieldOrder::Dmy, '/')
}

fn system_locale_tag() -> Option<String> {
    // Cross-platform, dependency-free best-effort.
    //
    // Common on Unix: LANG/LC_ALL like "en_US.UTF-8".
    // On Windows this is often unset; callers can/should override via `MaterialLocale`.
    for key in ["LC_ALL", "LANG", "LANGUAGE"] {
        if let Ok(value) = std::env::var(key) {
            let value = value.trim();
            if value.is_empty() {
                continue;
            }

            let without_encoding = value.split('.').next().unwrap_or(value);
            let tag = without_encoding.replace('_', "-");
            if !tag.is_empty() {
                return Some(tag);
            }
        }
    }

    None
}
