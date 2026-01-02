//! Translations (i18n) view for the showcase application.
//!
//! This view demonstrates:
//! - Scanning `.mui_lang` language files from `assets/i18n/`
//! - Selecting active language at runtime via dropdown
//! - Live-updating localized strings in UI components

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::select::{SelectBuilder, SpawnSelectChild};

use crate::showcase::common::*;

// ---------------------------------------------------------------------------
// Markers / IDs
// ---------------------------------------------------------------------------

#[derive(Component)]
pub struct TranslationsNewFileNameField;

#[derive(Component)]
pub struct TranslationsCreateFileButton;

#[derive(Component)]
pub struct TranslationsSaveFileButton;

#[derive(Component)]
pub struct TranslationKeyFieldLabel;

#[derive(Component)]
pub struct TranslationKeyFieldPlaceholder;

#[derive(Component)]
pub struct TranslationKeyFieldSupporting;

/// Spawn the translations section content.
pub fn spawn_translations_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section,
                theme,
                "showcase.section.translations.title",
                "Translations",
                "showcase.section.translations.description",
                "Switch languages at runtime",
            );

            // Language file selection.
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(24.0),
                    row_gap: Val::Px(16.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Dropdown (options are populated dynamically by showcase systems).
                    row.spawn_select_with(
                        theme,
                        SelectBuilder::new(vec![])
                            .outlined()
                            .label_key("showcase.translations.language_file"),
                    );
                });

            // Live demo UI that updates when the language changes.
            section
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    ..default()
                })
                .with_children(|editor| {
                    editor.spawn((
                        Text::new(""),
                        LocalizedText::new("showcase.translations.greeting").with_default("Hello!"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));

                    // Button with localized label.
                    editor
                        .spawn((material_button_bundle(theme, "", ButtonVariant::Filled),))
                        .with_children(|btn| {
                            btn.spawn((
                                bevy_material_ui::button::ButtonLabel,
                                Text::new(""),
                                LocalizedText::new("showcase.translations.button")
                                    .with_default("Click me"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.on_primary),
                            ));
                        });

                    // Text field using translation keys.
                    editor.spawn_text_field_with(
                        theme,
                        TextFieldBuilder::new()
                            .label_key("showcase.text_fields.email.label")
                            .placeholder_key("showcase.text_fields.email.placeholder")
                            .supporting_text_key("showcase.text_fields.email.supporting")
                            .outlined()
                            .width(Val::Px(260.0)),
                    );

                    spawn_code_block(
                        editor,
                        theme,
                        r#"// Files live in assets/i18n/*.mui_lang
// Switching the dropdown updates MaterialLanguage.tag.
// Text nodes: attach `LocalizedText { key: ... }`.
// Text fields: use `.label_key(..)`, `.placeholder_key(..)`, `.supporting_text_key(..)`."#,
                    );
                });
        });
}
