//! Time Picker Demo
//!
//! Demonstrates the Material Design 3 time picker component.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

#[derive(Component)]
struct OpenPickerButton(Entity);

#[derive(Component)]
struct ResultText(Entity);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, time_picker_demo_system)
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            Transform::default(),
            GlobalTransform::default(),
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("time_picker_demo/root", &telemetry)
        .with_children(|root| {
            // Picker overlay (hidden until opened)
            let picker_entity = root.spawn_time_picker(
                &theme,
                TimePickerBuilder::new()
                    .title("Select Time")
                    .initial_time(13, 30)
                    .format(TimeFormat::H24)
                    .width(Val::Px(360.0)),
            );

            // Open button
            let label = "Open Time Picker";
            let btn = MaterialButton::new(label).with_variant(ButtonVariant::Filled);
            let label_color = btn.text_color(&theme);

            root.spawn((
                OpenPickerButton(picker_entity),
                Interaction::None,
                MaterialButtonBuilder::new(label).filled().build(&theme),
            ))
            .insert_test_id("time_picker_demo/open", &telemetry)
            .with_children(|b| {
                b.spawn((
                    ButtonLabel,
                    Text::new(label),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(label_color),
                ));
            });

            // Result display
            root.spawn((
                ResultText(picker_entity),
                Text::new("Result: None"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ))
            .insert_test_id("time_picker_demo/result", &telemetry);
        });
}

#[allow(clippy::type_complexity)]
fn time_picker_demo_system(
    mut open_buttons: Query<(&Interaction, &OpenPickerButton), Changed<Interaction>>,
    mut pickers: Query<&mut MaterialTimePicker>,
    mut submit: MessageReader<TimePickerSubmitEvent>,
    mut cancel: MessageReader<TimePickerCancelEvent>,
    mut result_texts: Query<(&ResultText, &mut Text)>,
) {
    // Open picker when button is pressed
    for (interaction, open_button) in open_buttons.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(mut picker) = pickers.get_mut(open_button.0) {
            picker.open = true;
        }
    }

    // Update result text on submit
    for ev in submit.read() {
        let label = format!("Result: {:02}:{:02}", ev.hour, ev.minute);

        for (display, mut text) in result_texts.iter_mut() {
            if display.0 == ev.entity {
                *text = Text::new(label.as_str());
            }
        }
    }

    // Update result text on cancel
    for ev in cancel.read() {
        let label = if let Ok(picker) = pickers.get(ev.entity) {
            format!("Result: {:02}:{:02}", picker.hour, picker.minute)
        } else {
            "Result: Canceled".to_string()
        };

        for (display, mut text) in result_texts.iter_mut() {
            if display.0 == ev.entity {
                *text = Text::new(label.as_str());
            }
        }
    }
}
