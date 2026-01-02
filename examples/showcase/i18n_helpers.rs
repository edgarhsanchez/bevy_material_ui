//! Helper functions for spawning i18n-aware showcase components
//!
//! These functions replicate the library's spawn helpers but add LocalizedText
//! to labels for internationalization support.

use crate::showcase::common::NeedsInternationalFont;
use bevy::prelude::*;
use bevy_material_ui::checkbox::{CheckboxBox, CheckboxIcon, CheckboxStateLayer};
use bevy_material_ui::chip::ChipLabel;
use bevy_material_ui::icons::{icon_by_name, MaterialIcon, ICON_CHECK};
use bevy_material_ui::prelude::*;
use bevy_material_ui::radio::{RadioInner, RadioOuter, RadioStateLayer};
use bevy_material_ui::switch::SwitchHandle;

// Constants from library
const CHECKBOX_TOUCH_TARGET: f32 = 40.0;
const CHECKBOX_SIZE: f32 = 18.0;
const CHECKBOX_BORDER_WIDTH: f32 = 2.0;
const CHECKBOX_CORNER_RADIUS: f32 = 2.0;

const SWITCH_TRACK_WIDTH: f32 = 52.0;
const SWITCH_TRACK_HEIGHT: f32 = 32.0;

const RADIO_TOUCH_TARGET: f32 = 40.0;
const RADIO_SIZE: f32 = 20.0;
const RADIO_DOT_SIZE: f32 = 10.0;

// CHECKBOX I18N HELPERS

pub fn spawn_checkbox_i18n(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    state: CheckboxState,
    translation_key: &str,
    default_text: &str,
) {
    let checkbox = MaterialCheckbox::new().with_state(state);
    let bg_color = checkbox.container_color(theme);
    let border_color = checkbox.outline_color(theme);
    let icon_color = checkbox.icon_color(theme);
    let state_layer_color = checkbox.state_layer_color(theme);
    let icon_name = checkbox.state.icon();
    let default_icon_id = icon_by_name(ICON_CHECK).expect("embedded icon 'check' not found");
    let icon_visibility = if icon_name.is_some() {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    let icon_id = icon_name.and_then(icon_by_name).unwrap_or(default_icon_id);

    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|row| {
            // Checkbox
            row.spawn((
                checkbox,
                Button,
                Interaction::None,
                RippleHost::new(),
                Node {
                    width: Val::Px(CHECKBOX_TOUCH_TARGET),
                    height: Val::Px(CHECKBOX_TOUCH_TARGET),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderRadius::all(Val::Px(20.0)),
            ))
            .with_children(|parent| {
                // State layer
                parent
                    .spawn((
                        CheckboxStateLayer,
                        StateLayer::new(state_layer_color),
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(20.0)),
                    ))
                    .with_children(|state_layer_parent| {
                        // Checkbox box
                        state_layer_parent
                            .spawn((
                                CheckboxBox,
                                Node {
                                    width: Val::Px(CHECKBOX_SIZE),
                                    height: Val::Px(CHECKBOX_SIZE),
                                    border: UiRect::all(Val::Px(CHECKBOX_BORDER_WIDTH)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(bg_color),
                                BorderColor::all(border_color),
                                BorderRadius::all(Val::Px(CHECKBOX_CORNER_RADIUS)),
                            ))
                            .with_children(|box_parent| {
                                // Checkmark
                                box_parent.spawn((
                                    CheckboxIcon,
                                    MaterialIcon::new(icon_id)
                                        .with_size(14.0)
                                        .with_color(icon_color),
                                    icon_visibility,
                                ));
                            });
                    });
            });

            // Label with localization
            row.spawn((
                Text::new(""),
                LocalizedText::new(translation_key).with_default(default_text),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.on_surface),
                NeedsInternationalFont,
            ));
        });
}

// SWITCH I18N HELPERS

pub fn spawn_switch_i18n(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    selected: bool,
    translation_key: &str,
    default_text: &str,
) {
    let switch = MaterialSwitch::new().selected(selected);
    let bg_color = switch.track_color(theme);
    let border_color = switch.track_outline_color(theme);
    let handle_color = switch.handle_color(theme);
    let handle_size = switch.handle_size();
    let has_border = !selected;
    let justify = if selected {
        JustifyContent::FlexEnd
    } else {
        JustifyContent::FlexStart
    };

    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|row| {
            // Switch track (main touch target)
            row.spawn((
                switch,
                Button,
                Interaction::None,
                RippleHost::new(),
                Node {
                    width: Val::Px(SWITCH_TRACK_WIDTH),
                    height: Val::Px(SWITCH_TRACK_HEIGHT),
                    justify_content: justify,
                    align_items: AlignItems::Center,
                    padding: UiRect::horizontal(Val::Px(2.0)),
                    border: UiRect::all(Val::Px(if has_border { 2.0 } else { 0.0 })),
                    ..default()
                },
                BackgroundColor(bg_color),
                BorderColor::all(border_color),
                BorderRadius::all(Val::Px(16.0)),
            ))
            .with_children(|track| {
                // Handle (thumb)
                track.spawn((
                    SwitchHandle,
                    Node {
                        width: Val::Px(handle_size),
                        height: Val::Px(handle_size),
                        ..default()
                    },
                    BackgroundColor(handle_color),
                    BorderRadius::all(Val::Px(handle_size / 2.0)),
                ));
            });

            // Label with localization
            row.spawn((
                Text::new(""),
                LocalizedText::new(translation_key).with_default(default_text),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.on_surface),
                NeedsInternationalFont,
            ));
        });
}

// RADIO I18N HELPERS

pub fn spawn_radio_i18n(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    selected: bool,
    group: impl Into<String>,
    translation_key: &str,
    default_text: &str,
) {
    let radio = MaterialRadio::new().selected(selected).group(group);
    let border_color = radio.outer_color(theme);
    let inner_color = if selected { theme.primary } else { Color::NONE };
    let state_layer_color = radio.state_layer_color(theme);

    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|row| {
            // Radio touch target
            row.spawn((
                radio,
                Button,
                Interaction::None,
                RippleHost::new(),
                Node {
                    width: Val::Px(RADIO_TOUCH_TARGET),
                    height: Val::Px(RADIO_TOUCH_TARGET),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                BorderRadius::all(Val::Px(20.0)),
            ))
            .with_children(|touch| {
                // State layer
                touch
                    .spawn((
                        RadioStateLayer,
                        StateLayer::new(state_layer_color),
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                        BorderRadius::all(Val::Px(20.0)),
                    ))
                    .with_children(|state_layer| {
                        // Outer circle
                        state_layer
                            .spawn((
                                RadioOuter,
                                Node {
                                    width: Val::Px(RADIO_SIZE),
                                    height: Val::Px(RADIO_SIZE),
                                    border: UiRect::all(Val::Px(2.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                                BorderColor::all(border_color),
                                BorderRadius::all(Val::Px(RADIO_SIZE / 2.0)),
                            ))
                            .with_children(|outer| {
                                // Inner dot
                                outer.spawn((
                                    RadioInner,
                                    Node {
                                        width: Val::Px(RADIO_DOT_SIZE),
                                        height: Val::Px(RADIO_DOT_SIZE),
                                        ..default()
                                    },
                                    BackgroundColor(inner_color),
                                    BorderRadius::all(Val::Px(RADIO_DOT_SIZE / 2.0)),
                                ));
                            });
                    });
            });

            // Label with localization
            row.spawn((
                Text::new(""),
                LocalizedText::new(translation_key).with_default(default_text),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.on_surface),
                NeedsInternationalFont,
            ));
        });
}

// CHIP I18N HELPERS

pub fn spawn_chip_i18n(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    translation_key: &str,
    default_text: &str,
    selected: bool,
) {
    // Create a filter chip builder
    let chip = MaterialChip::filter(default_text).with_selected(selected);
    let label_color = chip.label_color(theme);
    let icon_color = chip.icon_color(theme);

    parent
        .spawn((
            chip,
            Button,
            Interaction::None,
            RippleHost::new(),
            Node {
                height: Val::Px(32.0),
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(16.0)),
                column_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(if selected {
                theme.secondary_container
            } else {
                Color::NONE
            }),
            BorderRadius::all(Val::Px(8.0)),
        ))
        .with_children(|chip_parent| {
            // Leading checkmark for selected chips
            if selected {
                chip_parent.spawn((
                    Text::new("✓"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(icon_color),
                ));
            }

            // Label with localization
            chip_parent.spawn((
                ChipLabel,
                Text::new(""),
                LocalizedText::new(translation_key).with_default(default_text),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(label_color),
                NeedsInternationalFont,
            ));
        });
}

// FAB I18N HELPERS

pub fn spawn_extended_fab_i18n(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    icon: impl Into<String>,
    translation_key: &str,
    default_text: &str,
) {
    use bevy_material_ui::fab::{FabLabel, MaterialFab};
    use bevy_material_ui::icons::{IconStyle, MaterialIcon};

    let icon_name = icon.into();
    let fab = MaterialFab::new(icon_name.clone()).extended(default_text);
    let icon_color = fab.content_color(theme);
    let text_color = fab.content_color(theme);
    let icon_size = fab.size.icon_size();

    parent
        .spawn((
            fab,
            Button,
            Interaction::None,
            RippleHost::new(),
            Node {
                height: Val::Px(56.0),
                padding: UiRect::horizontal(Val::Px(16.0)),
                align_items: AlignItems::Center,
                column_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(theme.primary_container),
            BorderRadius::all(Val::Px(16.0)),
        ))
        .with_children(|fab_parent| {
            if let Some(icon) = MaterialIcon::from_name(&icon_name) {
                fab_parent.spawn((
                    icon,
                    IconStyle::outlined()
                        .with_color(icon_color)
                        .with_size(icon_size),
                ));
            }
            fab_parent.spawn((
                FabLabel,
                Text::new(""),
                LocalizedText::new(translation_key).with_default(default_text),
                TextColor(text_color),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                NeedsInternationalFont,
            ));
        });
}
