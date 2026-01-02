//! Dialog Demo
//!
//! Demonstrates Material Design 3 dialogs.

use bevy::prelude::*;
use bevy_material_ui::dialog::create_dialog_scrim_for;
use bevy_material_ui::prelude::*;

#[derive(Component)]
struct OpenDialogButton;

#[derive(Component)]
struct CancelDialogButton;

#[derive(Component)]
struct ConfirmDialogButton;

#[derive(Resource)]
struct DialogEntities {
    dialog: Entity,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (open_dialog_system, close_dialog_system))
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
                padding: UiRect::all(Val::Px(24.0)),
                row_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("dialog_demo/root", &telemetry)
        .with_children(|root| {
            let open_label = "Open Dialog";
            let open_button = MaterialButton::new(open_label).with_variant(ButtonVariant::Filled);
            let open_text_color = open_button.text_color(&theme);

            root.spawn((
                OpenDialogButton,
                Interaction::None,
                MaterialButtonBuilder::new(open_label)
                    .filled()
                    .build(&theme),
            ))
            .insert_test_id("dialog_demo/open_button", &telemetry)
            .with_children(|btn| {
                btn.spawn((
                    ButtonLabel,
                    Text::new(open_label),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(open_text_color),
                ));
            });
        });

    let dialog_entity = commands
        .spawn((
            GlobalZIndex(1001),
            DialogBuilder::new()
                .title("Confirm Action")
                .modal(true)
                .build(&theme),
        ))
        .insert_test_id("dialog_demo/dialog", &telemetry)
        .with_children(|dialog| {
            dialog.spawn((
                Text::new("Are you sure you want to proceed?"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
                Node {
                    margin: UiRect::bottom(Val::Px(16.0)),
                    ..default()
                },
            ));

            dialog
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::End,
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    DialogActions,
                ))
                .with_children(|actions| {
                    let cancel_label = "Cancel";
                    actions
                        .spawn((
                            CancelDialogButton,
                            Interaction::None,
                            MaterialButtonBuilder::new(cancel_label)
                                .text()
                                .build(&theme),
                        ))
                        .insert_test_id("dialog_demo/dialog/cancel", &telemetry)
                        .with_children(|btn| {
                            btn.spawn((
                                ButtonLabel,
                                Text::new(cancel_label),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(theme.primary),
                            ));
                        });

                    let confirm_label = "Confirm";
                    let confirm_button =
                        MaterialButton::new(confirm_label).with_variant(ButtonVariant::Filled);
                    let confirm_text_color = confirm_button.text_color(&theme);

                    actions
                        .spawn((
                            ConfirmDialogButton,
                            Interaction::None,
                            MaterialButtonBuilder::new(confirm_label)
                                .filled()
                                .build(&theme),
                        ))
                        .insert_test_id("dialog_demo/dialog/confirm", &telemetry)
                        .with_children(|btn| {
                            btn.spawn((
                                ButtonLabel,
                                Text::new(confirm_label),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(confirm_text_color),
                            ));
                        });
                });
        })
        .id();

    // Scrim follows dialog open state and modality.
    commands
        .spawn(create_dialog_scrim_for(&theme, dialog_entity, true))
        .insert_test_id("dialog_demo/dialog/scrim", &telemetry);

    commands.insert_resource(DialogEntities {
        dialog: dialog_entity,
    });
}

fn open_dialog_system(
    entities: Res<DialogEntities>,
    mut interactions: Query<&Interaction, (Changed<Interaction>, With<OpenDialogButton>)>,
    mut dialogs: Query<&mut MaterialDialog>,
) {
    let Ok(mut dialog) = dialogs.get_mut(entities.dialog) else {
        return;
    };

    for interaction in interactions.iter_mut() {
        if *interaction == Interaction::Pressed {
            dialog.open = true;
        }
    }
}

fn close_dialog_system(
    entities: Res<DialogEntities>,
    mut dialogs: Query<&mut MaterialDialog>,
    mut cancel: Query<&Interaction, (Changed<Interaction>, With<CancelDialogButton>)>,
    mut confirm: Query<&Interaction, (Changed<Interaction>, With<ConfirmDialogButton>)>,
) {
    let Ok(mut dialog) = dialogs.get_mut(entities.dialog) else {
        return;
    };

    let should_close = cancel.iter_mut().any(|i| *i == Interaction::Pressed)
        || confirm.iter_mut().any(|i| *i == Interaction::Pressed);

    if should_close {
        dialog.open = false;
    }
}
