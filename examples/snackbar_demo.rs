//! Snackbar Demo
//!
//! Demonstrates snackbar host positioning and triggering snackbars via messages.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

#[derive(Component)]
struct ShowSnackbarButton;

#[derive(Component)]
struct ShowActionSnackbarButton;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (show_snackbar_on_click_system, handle_snackbar_action_system),
        )
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
    commands.spawn(Camera2d);

    // Root container
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
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("snackbar_demo/root", &telemetry)
        .with_children(|root| {
            // Host lives in the UI tree; snackbars are spawned under it.
            root.spawn_snackbar_host(SnackbarPosition::BottomCenter);

            // Buttons to trigger snackbars.
            let label = "Show Snackbar";
            let btn = MaterialButton::new(label).with_variant(ButtonVariant::Filled);
            let label_color = btn.text_color(&theme);

            root.spawn((
                ShowSnackbarButton,
                Interaction::None,
                MaterialButtonBuilder::new(label).filled().build(&theme),
            ))
            .insert_test_id("snackbar_demo/show", &telemetry)
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

            let label = "Show Snackbar (Action)";
            let btn = MaterialButton::new(label).with_variant(ButtonVariant::Outlined);
            let label_color = btn.text_color(&theme);

            root.spawn((
                ShowActionSnackbarButton,
                Interaction::None,
                MaterialButtonBuilder::new(label).outlined().build(&theme),
            ))
            .insert_test_id("snackbar_demo/show_action", &telemetry)
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
        });
}

fn show_snackbar_on_click_system(
    mut clicks: Query<
        (&Interaction, Option<&ShowActionSnackbarButton>),
        (
            Changed<Interaction>,
            Or<(With<ShowSnackbarButton>, With<ShowActionSnackbarButton>)>,
        ),
    >,
    mut show: MessageWriter<ShowSnackbar>,
) {
    for (interaction, action) in clicks.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if action.is_some() {
            show.write(ShowSnackbar::with_action("Saved successfully", "UNDO"));
        } else {
            show.write(ShowSnackbar::message("Hello from snackbar!"));
        }
    }
}

fn handle_snackbar_action_system(mut actions: MessageReader<SnackbarActionEvent>) {
    for ev in actions.read() {
        info!("Snackbar action clicked: {}", ev.action);
    }
}
