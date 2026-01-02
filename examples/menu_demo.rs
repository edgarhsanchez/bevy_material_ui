//! Menu Demo
//!
//! Demonstrates Material Design 3 menus.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

#[derive(Component)]
struct MenuTriggerButton;

#[derive(Component)]
struct DemoMenu;

#[derive(Component)]
struct DemoMenuItem;

#[derive(Resource)]
struct MenuEntities {
    menu: Entity,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_plugins(TelemetryPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                toggle_menu_system,
                close_menu_on_item_system,
                position_menu_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, telemetry: Res<TelemetryConfig>) {
    commands.spawn(Camera2d);

    let mut menu_items: Vec<(Entity, &'static str)> = Vec::new();
    let mut menu_entity: Option<Entity> = None;

    // Root + container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .insert_test_id("menu_demo/root", &telemetry)
        .with_children(|root| {
            root.spawn((
                Node {
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    row_gap: Val::Px(8.0),
                    padding: UiRect::all(Val::Px(24.0)),
                    ..default()
                },
                BackgroundColor(theme.surface_container_low),
                BorderRadius::all(Val::Px(12.0)),
            ))
            .with_children(|container| {
                // Trigger button.
                let trigger_label = "Open Menu";
                let trigger_button =
                    MaterialButton::new(trigger_label).with_variant(ButtonVariant::Outlined);
                let trigger_text_color = trigger_button.text_color(&theme);

                container
                    .spawn((
                        MenuTriggerButton,
                        Interaction::None,
                        MaterialButtonBuilder::new(trigger_label)
                            .outlined()
                            .build(&theme),
                    ))
                    .insert_test_id("menu_demo/trigger", &telemetry)
                    .with_children(|btn| {
                        btn.spawn((
                            ButtonLabel,
                            Text::new(trigger_label),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(trigger_text_color),
                        ));
                    });

                // Menu
                let id = container
                    .spawn((DemoMenu, MenuBuilder::new().build(&theme)))
                    .insert_test_id("menu_demo/menu", &telemetry)
                    .with_children(|menu| {
                        let cut = menu
                            .spawn((DemoMenuItem, MenuItemBuilder::new("Cut").build(&theme)))
                            .with_children(|row| {
                                row.spawn((
                                    Text::new("Cut"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface),
                                ));
                            })
                            .id();
                        menu_items.push((cut, "cut"));

                        let copy = menu
                            .spawn((DemoMenuItem, MenuItemBuilder::new("Copy").build(&theme)))
                            .with_children(|row| {
                                row.spawn((
                                    Text::new("Copy"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface),
                                ));
                            })
                            .id();
                        menu_items.push((copy, "copy"));

                        menu.spawn(create_menu_divider(&theme));

                        let paste = menu
                            .spawn((DemoMenuItem, MenuItemBuilder::new("Paste").build(&theme)))
                            .with_children(|row| {
                                row.spawn((
                                    Text::new("Paste"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface),
                                ));
                            })
                            .id();
                        menu_items.push((paste, "paste"));
                    })
                    .id();

                menu_entity = Some(id);
            });
        });

    let menu_entity = menu_entity.expect("menu entity should be spawned");
    commands.insert_resource(MenuEntities { menu: menu_entity });

    for (entity, key) in menu_items {
        commands
            .entity(entity)
            .insert_test_id(format!("menu_demo/menu/item/{key}"), &telemetry);
    }
}

fn toggle_menu_system(
    entities: Res<MenuEntities>,
    mut interactions: Query<&Interaction, (Changed<Interaction>, With<MenuTriggerButton>)>,
    mut menus: Query<&mut MaterialMenu>,
) {
    let Ok(mut menu) = menus.get_mut(entities.menu) else {
        return;
    };

    for interaction in interactions.iter_mut() {
        if *interaction == Interaction::Pressed {
            menu.open = !menu.open;
        }
    }
}

fn close_menu_on_item_system(
    entities: Res<MenuEntities>,
    mut menus: Query<&mut MaterialMenu>,
    mut interactions: Query<&Interaction, (Changed<Interaction>, With<DemoMenuItem>)>,
) {
    let should_close = interactions.iter_mut().any(|i| *i == Interaction::Pressed);
    if !should_close {
        return;
    }

    let Ok(mut menu) = menus.get_mut(entities.menu) else {
        return;
    };
    menu.open = false;
}

fn position_menu_system(entities: Res<MenuEntities>, mut nodes: Query<&mut Node, With<DemoMenu>>) {
    let Ok(mut node) = nodes.get_mut(entities.menu) else {
        return;
    };

    // A simple dropdown-style position.
    node.top = Val::Px(48.0 + 8.0);
    node.left = Val::Px(0.0);
}
