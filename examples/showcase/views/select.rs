//! Select dropdown view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::select::{SelectBuilder, SelectOption, SpawnSelectChild};

use crate::showcase::common::*;

/// Spawn the select section content.
pub fn spawn_select_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    _icon_font: Handle<Font>,
) {
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
                "showcase.section.select.title",
                "Select / Dropdown",
                "showcase.section.select.description",
                "MaterialSelect with options and selection events",
            );

            let options = vec![
                SelectOption::new("")
                    .label_key("showcase.select.option.1")
                    .value("opt1"),
                SelectOption::new("")
                    .label_key("showcase.select.option.2")
                    .value("opt2"),
                SelectOption::new("")
                    .label_key("showcase.select.option.3")
                    .value("opt3"),
            ];

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(24.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: Val::Px(16.0),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|col| {
                        col.spawn_select_with(
                            theme,
                            SelectBuilder::new(options.clone())
                                .filled()
                                .label_key("showcase.select.label.filled"),
                        );
                    });

                    row.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|col| {
                        col.spawn_select_with(
                            theme,
                            SelectBuilder::new(options.clone())
                                .outlined()
                                .label_key("showcase.select.label.outlined"),
                        );
                    });

                    row.spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|col| {
                        col.spawn_select_with(
                            theme,
                            SelectBuilder::new(options.clone())
                                .outlined()
                                .label_key("showcase.select.label.with_selection")
                                .selected(1),
                        );
                    });
                });

            spawn_code_block(
                section,
                theme,
                r#"use bevy_material_ui::select::{SelectBuilder, SelectOption};

let options = vec![
    SelectOption::new("Option 1").value("opt1"),
    SelectOption::new("Option 2").value("opt2"),
    SelectOption::new("Option 3").value("opt3"),
];

commands.spawn(
    SelectBuilder::new(options)
        .outlined()
        .label("Choose")
        .selected(0)
        .build(&theme),
);"#,
            );
        });
}
