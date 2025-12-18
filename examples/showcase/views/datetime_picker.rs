//! DateTime picker view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the date-time picker section content
pub fn spawn_datetime_picker_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "DateTime Picker",
                "Dialog-based date & time selection",
            );

            // Picker overlay (hidden until opened)
            let picker_entity = section.spawn_datetime_picker_entity_with(
                theme,
                DateTimePickerBuilder::new()
                    .title("Select date & time")
                    .date(Date::new(2025, 1, 15))
                    .time(13, 30)
                    .time_format(TimeFormat::H12)
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
                    let label = "Open Picker";
                    let btn = MaterialButton::new(label).with_variant(ButtonVariant::Filled);
                    let text_color = btn.text_color(theme);

                    row.spawn((
                        DateTimePickerOpenButton(picker_entity),
                        Interaction::None,
                        MaterialButtonBuilder::new(label).filled().build(theme),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            ButtonLabel,
                            Text::new(label),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(text_color),
                        ));
                    });

                    row.spawn((
                        DateTimePickerResultDisplay(picker_entity),
                        Text::new("Result: None"),
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
                r#"// Spawn a dialog-based date-time picker
section.spawn_datetime_picker_with(
    theme,
    DateTimePickerBuilder::new()
        .title(\"Select date & time\")
        .open()
        .min_date(Date::new(2025, 1, 1))
        .max_date(Date::new(2025, 12, 31))
);

// Listen for submit/cancel messages
fn handle_picker_events(
    mut submit: MessageReader<DateTimePickerSubmitEvent>,
    mut cancel: MessageReader<DateTimePickerCancelEvent>,
) {
    for ev in submit.read() {
        info!(\"Picked: {:?} {:02}:{:02}\", ev.date, ev.hour, ev.minute);
    }
    for _ in cancel.read() {
        info!(\"Picker canceled\");
    }
}"#,
            );
        });
}
