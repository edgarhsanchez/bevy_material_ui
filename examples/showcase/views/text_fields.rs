//! Text fields view for the showcase application.

use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::text_field::InputType;

use crate::showcase::common::*;

/// Spawn the text fields section content
pub fn spawn_text_fields_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
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
                "showcase.section.text_fields.title",
                "Text Fields",
                "showcase.section.text_fields.description",
                "Text input with Filled and Outlined variants",
            );

            section
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(24.0),
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: Val::Px(16.0),
                    margin: UiRect::vertical(Val::Px(8.0)),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn_text_field_with(
                        theme,
                        TextFieldBuilder::new()
                            .label("Filled")
                            .placeholder("Type here…")
                            .supporting_text("Click to focus and type")
                            .filled()
                            .width(Val::Px(240.0)),
                    );

                    row.spawn_text_field_with(
                        theme,
                        TextFieldBuilder::new()
                            .label("Outlined")
                            .placeholder("Type here…")
                            .supporting_text("Enter submits")
                            .outlined()
                            .width(Val::Px(240.0)),
                    );

                    row.spawn_text_field_with(
                        theme,
                        TextFieldBuilder::new()
                            .label("With Error")
                            .placeholder("Invalid input")
                            .error_text("This field has an error")
                            .filled()
                            .width(Val::Px(240.0)),
                    );

                    row.spawn_text_field_with(
                        theme,
                        TextFieldBuilder::new()
                            .label("Email")
                            .placeholder("name@example.com")
                            .supporting_text("Must look like name@example.com")
                            .label_key("showcase.text_fields.email.label")
                            .placeholder_key("showcase.text_fields.email.placeholder")
                            .supporting_text_key("showcase.text_fields.email.supporting")
                            .input_type(InputType::Email)
                            .outlined()
                            .width(Val::Px(240.0)),
                    );
                });

            spawn_code_block(
                section,
                theme,
                r#"// Create a text field
let text_field = MaterialTextField::new()
    .with_variant(TextFieldVariant::Outlined)
    .label("Email")
    .placeholder("Enter your email")
    .supporting_text("We'll never share your email");

commands.spawn((
    text_field,
    Node { width: Val::Px(280.0), ..default() },
));"#,
            );
        });
}
