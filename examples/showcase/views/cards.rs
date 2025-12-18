//! Cards view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;

use crate::showcase::common::*;

/// Spawn the cards section content
pub fn spawn_cards_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "Cards",
                "Containers for related content and actions",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    flex_wrap: FlexWrap::Wrap,
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    spawn_card(row, theme, "Elevated", CardType::Elevated);
                    spawn_card(row, theme, "Filled", CardType::Filled);
                    spawn_card(row, theme, "Outlined", CardType::Outlined);
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create an elevated card
let card = MaterialCard::new()
    .variant(CardVariant::Elevated);

// Filled card
let card = MaterialCard::filled();

// Outlined card  
let card = MaterialCard::outlined();"#,
            );
        });
}

#[derive(Clone, Copy)]
enum CardType {
    Elevated,
    Filled,
    Outlined,
}

fn spawn_card(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    title: &str,
    card_type: CardType,
) {
    let builder = match card_type {
        CardType::Elevated => CardBuilder::new().elevated(),
        CardType::Filled => CardBuilder::new().filled(),
        CardType::Outlined => CardBuilder::new().outlined(),
    }
    .width(Val::Px(160.0))
    .padding(16.0);

    parent
        .spawn((Interaction::None, builder.build(theme)))
        .with_children(|card| {
            card.spawn((
                Text::new(title),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));
            card.spawn((
                Text::new("Card content goes here with supporting text."),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));
        });
}
