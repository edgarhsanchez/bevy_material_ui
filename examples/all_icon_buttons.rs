//! Demo: render *all* embedded Material icons as icon buttons.
//!
//! Run with:
//! `cargo run --example all_icon_buttons --release`

use bevy::prelude::*;
use bevy_material_ui::icons::material_icons;
use bevy_material_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
    commands.spawn(Camera2d);

    info!("Embedded icons discovered: {}", material_icons::ALL.len());

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
                    overflow: Overflow::scroll(),
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
                        for (name, id) in material_icons::ALL {
                            let icon_name = (*name).to_string();
                            let icon_color = MaterialIconButton::new(icon_name.clone())
                                .with_variant(IconButtonVariant::Standard)
                                .icon_color(&theme);

                            grid.spawn((
                                IconButtonBuilder::new(icon_name.clone())
                                    .standard()
                                    .build(&theme),
                                TooltipTrigger::new(icon_name),
                            ))
                            .with_children(|btn| {
                                btn.spawn(
                                    bevy_material_ui::icons::MaterialIcon::new(*id)
                                        .with_size(24.0)
                                        .with_color(icon_color),
                                );
                            });
                        }
                    });
            });
        });
}
// (No font/codepoint scanning; icons are embedded bitmaps.)
