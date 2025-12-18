//! Demo: render *all* Material Symbols glyphs as Material icon buttons.
//!
//! Run with:
//! `cargo run --example all_icon_buttons --release`

use bevy::prelude::*;
use bevy_material_ui::icons::EMBEDDED_MATERIAL_SYMBOLS_FONT;
use bevy_material_ui::prelude::*;
use std::collections::HashMap;

const CODEPOINTS_RS: &str = include_str!("../src/icons/codepoints.rs");

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, icon_font: Res<MaterialIconFont>) {
    commands.spawn(Camera2d);

    let codepoints = collect_font_codepoints();
    let codepoint_names = collect_codepoint_names();
    info!(
        "Material Symbols glyphs discovered (PUA scan): {}",
        codepoints.len()
    );

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .with_children(|root| {
            root.spawn((
                ScrollContainer::vertical().with_scrollbars(true),
                ScrollPosition::default(),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    overflow: Overflow::scroll_y(),
                    ..default()
                },
            ))
            .with_children(|scroll| {
                // Important:
                // Don't spawn a `ScrollContent` node manually here.
                // The crate's scroll plugin will create an internal `ScrollContent` wrapper node
                // and move our actual content under it, keeping scrollbars as a non-scrolling
                // overlay.
                scroll
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        padding: UiRect::all(Val::Px(Spacing::MEDIUM)),
                        column_gap: Val::Px(Spacing::SMALL),
                        row_gap: Val::Px(Spacing::SMALL),
                        ..default()
                    })
                    .with_children(|grid| {
                        // One standard icon button per glyph.
                        //
                        // We render the glyph directly as `Text` with the icon font handle.
                        // This avoids depending on the `(MaterialIcon, IconStyle)` sync system,
                        // and proves the embedded font path works end-to-end.
                        for ch in codepoints {
                            let icon_str = ch.to_string();
                            let icon_color = MaterialIconButton::new(icon_str.clone())
                                .with_variant(IconButtonVariant::Standard)
                                .icon_color(&theme);

                            let tooltip_text = codepoint_names
                                .get(&ch)
                                .cloned()
                                .unwrap_or_else(|| format!("U+{:04X}", ch as u32));

                            grid.spawn((
                                IconButtonBuilder::new(icon_str).standard().build(&theme),
                                TooltipTrigger::new(tooltip_text),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new(ch.to_string()),
                                    TextFont {
                                        font: icon_font.0.clone(),
                                        font_size: 24.0,
                                        ..default()
                                    },
                                    TextColor(icon_color),
                                ));
                            });
                        }
                    });
            });
        });
}

fn collect_codepoint_names() -> HashMap<char, String> {
    // Build a best-effort map from codepoint -> icon name by parsing
    // `src/icons/codepoints.rs` at compile time.
    //
    // This example renders all glyphs in the embedded font, but the crate only
    // assigns names to a curated subset of them.
    let mut out: HashMap<char, String> = HashMap::new();

    for raw_line in CODEPOINTS_RS.lines() {
        let line = raw_line.trim();

        // Example line:
        // pub const ICON_HOME: char = '\u{E88A}';
        if !line.starts_with("pub const ICON_") {
            continue;
        }

        let Some((left, right)) = line.split_once(": char = ") else {
            continue;
        };

        let Some(const_name) = left.trim().strip_prefix("pub const ") else {
            continue;
        };

        let Some(name) = const_name.trim().strip_prefix("ICON_") else {
            continue;
        };

        let Some(hex) = right.split("\\u{").nth(1).and_then(|s| s.split('}').next()) else {
            continue;
        };

        let Ok(codepoint) = u32::from_str_radix(hex, 16) else {
            continue;
        };

        let Some(ch) = char::from_u32(codepoint) else {
            continue;
        };

        out.insert(ch, name.to_lowercase());
    }

    out
}

fn collect_font_codepoints() -> Vec<char> {
    // Parse the embedded font bytes and collect all Unicode codepoints that map to a glyph.
    //
    // Material Symbols primarily uses the Private Use Area(s). Scanning the full BMP is
    // unnecessary and can be confusing (many fonts have glyphs for lots of chars).
    let face = ttf_parser::Face::parse(EMBEDDED_MATERIAL_SYMBOLS_FONT, 0)
        .expect("Failed to parse embedded Material Symbols font");

    let mut out: Vec<char> = Vec::new();

    // Scan BMP Private Use Area.
    for u in 0xE000u32..=0xF8FFu32 {
        let Some(ch) = char::from_u32(u) else {
            continue;
        };
        if face.glyph_index(ch).is_some() {
            out.push(ch);
        }
    }

    // Scan a common supplementary PUA range (Material symbols are typically BMP PUA, but
    // this keeps us honest if the font ever adds glyphs there).
    for u in 0xF0000u32..=0xF8FFFu32 {
        let Some(ch) = char::from_u32(u) else {
            continue;
        };
        if face.glyph_index(ch).is_some() {
            out.push(ch);
        }
    }

    out
}
