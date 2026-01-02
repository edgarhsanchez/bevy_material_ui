//! Time Picker view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the time picker section content
pub fn spawn_time_picker_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        })
        .with_children(|section| {
            spawn_section_header(
                section,
                theme,
                "showcase.section.time_picker.title",
                "Time Picker",
                "showcase.section.time_picker.description",
                "Material Design 3 clock-based time selection",
            );

            // Picker overlay (hidden until opened)
            let picker_entity = section.spawn_time_picker(
                theme,
                TimePickerBuilder::new()
                    .title_key("showcase.time_picker.title")
                    .initial_time(13, 30)
                    .format(TimeFormat::H24)
                    .width(Val::Px(360.0)),
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|row| {
                    let btn = MaterialButton::new("").with_variant(ButtonVariant::Filled);
                    let text_color = btn.text_color(theme);

                    row.spawn((
                        TimePickerOpenButton(picker_entity),
                        Interaction::None,
                        MaterialButtonBuilder::new("").filled().build(theme),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            ButtonLabel,
                            Text::new(""),
                            LocalizedText::new("showcase.time_picker.open_button")
                                .with_default("Open Time Picker"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(text_color),
                        ));
                    });

                    row.spawn((
                        TimePickerResultDisplay(picker_entity),
                        Text::new(""),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));
                });

            spawn_code_block(
                section,
                theme,
                r#"// Spawn a Material Design 3 time picker
section.spawn_time_picker(
    theme,
    TimePickerBuilder::new()
        .title("Select Time")
        .initial_time(13, 30)
        .format(TimeFormat::H24)
);

// Listen for submit/cancel messages
fn handle_picker_events(
    mut submit: MessageReader<TimePickerSubmitEvent>,
    mut cancel: MessageReader<TimePickerCancelEvent>,
) {
    for ev in submit.read() {
        info!("Selected: {:02}:{:02}", ev.hour, ev.minute);
    }
    for _ in cancel.read() {
        info!("Picker canceled");
    }
}"#,
            );
        });
}
