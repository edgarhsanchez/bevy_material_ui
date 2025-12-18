#[path = "common.rs"]
pub mod common;

#[path = "navigation.rs"]
pub mod navigation;

#[path = "views/mod.rs"]
pub mod views;

use bevy::asset::{AssetPlugin, RenderAssetUsages};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::ui::{ComputedNode, OverflowAxis, PositionType, ScrollPosition, UiGlobalTransform};
use bevy_material_ui::loading_indicator::ShapeMorphMaterial;
use bevy_material_ui::prelude::*;
use bevy_material_ui::text_field::InputType;
use bevy_material_ui::theme::ThemeMode;
use std::collections::HashMap;
use std::path::PathBuf;

use common::*;
use navigation::*;
use views::*;

use bevy_material_ui::list::MaterialListItem;

#[derive(Component)]
struct SpinningDice;

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct SidebarNavScroll;

#[derive(Component)]
struct MainContentScroll;

#[derive(Component)]
struct DetailSurface;

#[derive(Resource, Default)]
struct ThemeRebuildGate {
    initialized: bool,
}

#[derive(Resource, Default)]
struct LayoutRebuildGate {
    initialized: bool,
    last_width: Option<WindowWidthClass>,
    last_height: Option<WindowHeightClass>,
}

#[derive(Resource)]
struct ListDemoOptions {
    mode: ListSelectionMode,
}

impl Default for ListDemoOptions {
    fn default() -> Self {
        Self {
            mode: ListSelectionMode::Single,
        }
    }
}

#[derive(Resource)]
struct DialogDemoOptions {
    position: DialogPosition,
    modal: bool,
}

impl Default for DialogDemoOptions {
    fn default() -> Self {
        Self {
            position: DialogPosition::default(),
            modal: true,
        }
    }
}

#[derive(Resource, Default)]
struct ShowcaseTextFieldCache {
    // Keyed by label text (sufficient for the showcase’s fixed set of fields).
    by_label: HashMap<String, String>,
}

/// Stores tab selections across UI rebuilds
#[derive(Resource, Default)]
pub struct TabStateCache {
    /// Maps section name to selected tab index
    pub selections: HashMap<ComponentSection, usize>,
}

pub fn run() {
    let asset_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: asset_root.to_string_lossy().to_string(),
            ..default()
        }))
        .add_plugins(MaterialUiPlugin)
        .init_resource::<ShowcaseThemeSelection>()
        // Default seed theme (Material You purple)
        .insert_resource(MaterialTheme::from_seed(
            Color::srgb_u8(0x67, 0x50, 0xA4),
            ThemeMode::Dark,
        ))
        .init_resource::<SelectedSection>()
        .init_resource::<ComponentTelemetry>()
        .init_resource::<SnackbarDemoOptions>()
        .init_resource::<TooltipDemoOptions>()
        .init_resource::<ListDemoOptions>()
        .init_resource::<DialogDemoOptions>()
        .init_resource::<ShowcaseTextFieldCache>()
        .init_resource::<TabStateCache>()
        .init_resource::<ThemeRebuildGate>()
        .init_resource::<LayoutRebuildGate>()
        .add_systems(Startup, (setup_3d_scene, setup_ui, setup_telemetry))
        // Restore cached values before the MaterialUiPlugin text-field systems run.
        .add_systems(PreUpdate, restore_showcase_text_field_values_system)
        .add_systems(
            Update,
            (
                rotate_dice,
                handle_nav_clicks,
                update_nav_highlights,
                update_detail_content,
                progress_demo_animate_system,
                demo_click_log_system,
                snackbar_demo_options_system,
                snackbar_demo_trigger_system,
                snackbar_demo_style_system,
                snackbar_demo_action_log_system,
                tooltip_demo_options_system,
                tooltip_demo_apply_system,
                tooltip_demo_style_system,
                menu_demo_system,
                datetime_picker_demo_system,
            ),
        )
        .add_systems(
            Update,
            (
                sidebar_scroll_telemetry_system,
                main_scroll_telemetry_system,
            ),
        )
        .add_systems(Update, email_validation_system)
        // Cache changes after UI input has been processed.
        .add_systems(PostUpdate, cache_showcase_text_field_changes_system)
        .add_systems(
            Update,
            (
                dialog_demo_position_options_system,
                dialog_demo_position_style_system,
                dialog_demo_apply_position_system,
                dialog_demo_modal_options_system,
                dialog_demo_modal_style_system,
                dialog_demo_apply_modal_system,
                dialog_demo_open_close_system,
                list_demo_mode_options_system,
                list_demo_mode_style_system,
                list_demo_apply_selection_mode_system,
                theme_mode_option_system,
                theme_seed_option_system,
                attach_theme_seed_text_field_system.before(theme_seed_text_field_system),
                theme_seed_text_field_system
                    .before(rebuild_ui_on_theme_change_system)
                    .before(rebuild_ui_on_size_class_change_system),
                cache_tab_state_system
                    .before(rebuild_ui_on_theme_change_system)
                    .before(rebuild_ui_on_size_class_change_system),
                rebuild_ui_on_theme_change_system,
                rebuild_ui_on_size_class_change_system,
            ),
        )
        .add_systems(
            Update,
            (
                ensure_automation_test_ids_clickables_system,
                ensure_automation_test_ids_inputs_system,
                ensure_automation_test_ids_overlays_system,
                telemetry_from_component_events_system,
                telemetry_list_selection_state_system,
                telemetry_snapshot_system,
                write_telemetry,
            ),
        )
        .run();
}

#[derive(Debug)]
struct InsertTestIdIfExists {
    entity: Entity,
    test_id: TestId,
}

impl Command for InsertTestIdIfExists {
    fn apply(self, world: &mut World) {
        if let Ok(mut entity) = world.get_entity_mut(self.entity) {
            // Only insert if still missing; entity may have been rebuilt.
            if entity.get::<TestId>().is_none() {
                entity.insert(self.test_id);
            }
        }
    }
}

fn ensure_automation_test_ids_clickables_system(
    selected: Res<SelectedSection>,
    telemetry: Res<ComponentTelemetry>,
    mut commands: Commands,
    buttons: Query<(Entity, &UiGlobalTransform), (With<MaterialButton>, Without<TestId>)>,
    chips: Query<(Entity, &UiGlobalTransform), (With<MaterialChip>, Without<TestId>)>,
    fabs: Query<(Entity, &UiGlobalTransform), (With<MaterialFab>, Without<TestId>)>,
    badges: Query<(Entity, &UiGlobalTransform), (With<MaterialBadge>, Without<TestId>)>,
    progress_linear: Query<
        (Entity, &UiGlobalTransform),
        (With<MaterialLinearProgress>, Without<TestId>),
    >,
    progress_circular: Query<
        (Entity, &UiGlobalTransform),
        (With<MaterialCircularProgress>, Without<TestId>),
    >,
    cards: Query<(Entity, &UiGlobalTransform), (With<MaterialCard>, Without<TestId>)>,
    dividers: Query<(Entity, &UiGlobalTransform), (With<MaterialDivider>, Without<TestId>)>,
    icons: Query<(Entity, &UiGlobalTransform), (With<MaterialIcon>, Without<TestId>)>,
    icon_buttons: Query<(Entity, &UiGlobalTransform), (With<MaterialIconButton>, Without<TestId>)>,
) {
    if !telemetry.enabled {
        return;
    }

    match selected.current {
        ComponentSection::Buttons => {
            let mut items: Vec<(Entity, f32)> =
                buttons.iter().map(|(e, t)| (e, t.translation.y)).collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("button_{}", i)),
                });
            }
        }
        ComponentSection::AppBar => {
            let mut icons: Vec<(Entity, f32)> = icon_buttons
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            icons.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in icons.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("app_bar_icon_{}", i)),
                });
            }

            let mut fab_items: Vec<(Entity, f32)> =
                fabs.iter().map(|(e, t)| (e, t.translation.y)).collect();
            fab_items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in fab_items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("app_bar_fab_{}", i)),
                });
            }
        }
        ComponentSection::Chips => {
            let mut items: Vec<(Entity, f32)> =
                chips.iter().map(|(e, t)| (e, t.translation.y)).collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("chip_{}", i)),
                });
            }
        }
        ComponentSection::Fab => {
            let mut items: Vec<(Entity, f32)> =
                fabs.iter().map(|(e, t)| (e, t.translation.y)).collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("fab_{}", i)),
                });
            }
        }
        ComponentSection::Badges => {
            let mut items: Vec<(Entity, f32)> =
                badges.iter().map(|(e, t)| (e, t.translation.y)).collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("badge_{}", i)),
                });
            }
        }
        ComponentSection::Progress => {
            let mut linear: Vec<(Entity, f32)> = progress_linear
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            linear.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in linear.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("progress_linear_{}", i)),
                });
            }

            let mut circular: Vec<(Entity, f32)> = progress_circular
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            circular.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in circular.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("progress_circular_{}", i)),
                });
            }
        }
        ComponentSection::Cards => {
            let mut items: Vec<(Entity, f32)> =
                cards.iter().map(|(e, t)| (e, t.translation.y)).collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("card_{}", i)),
                });
            }
        }
        ComponentSection::Dividers => {
            let mut items: Vec<(Entity, f32)> =
                dividers.iter().map(|(e, t)| (e, t.translation.y)).collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("divider_{}", i)),
                });
            }
        }
        ComponentSection::Icons => {
            let mut items: Vec<(Entity, f32)> =
                icons.iter().map(|(e, t)| (e, t.translation.y)).collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("icon_{}", i)),
                });
            }
        }
        ComponentSection::IconButtons => {
            let mut items: Vec<(Entity, f32)> = icon_buttons
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("icon_button_{}", i)),
                });
            }
        }
        _ => {}
    }
}

fn cache_showcase_text_field_changes_system(
    mut changes: MessageReader<TextFieldChangeEvent>,
    fields: Query<&MaterialTextField>,
    mut cache: ResMut<ShowcaseTextFieldCache>,
) {
    for ev in changes.read() {
        let Ok(field) = fields.get(ev.entity) else {
            continue;
        };

        let Some(label) = field.label.as_deref() else {
            continue;
        };

        cache.by_label.insert(label.to_string(), ev.value.clone());
    }
}

fn restore_showcase_text_field_values_system(
    mut new_fields: Query<&mut MaterialTextField, Added<MaterialTextField>>,
    cache: Res<ShowcaseTextFieldCache>,
) {
    if cache.by_label.is_empty() {
        return;
    }

    for mut field in new_fields.iter_mut() {
        let Some(label) = field.label.clone() else {
            continue;
        };

        let Some(value) = cache.by_label.get(&label) else {
            continue;
        };

        // Rehydrate the field’s value after an adaptive-layout rebuild.
        field.value = value.clone();
        field.has_content = !field.value.is_empty();

        // Keep the demo’s email validation behavior consistent after restore.
        if field.input_type == InputType::Email {
            let valid = is_valid_email(&field.value);
            if valid {
                field.error = false;
                field.error_text = None;
            } else {
                field.error = true;
                field.error_text = Some("Invalid email address".to_string());
            }
        }
    }
}

fn ensure_automation_test_ids_inputs_system(
    selected: Res<SelectedSection>,
    telemetry: Res<ComponentTelemetry>,
    mut commands: Commands,
    checkboxes: Query<(Entity, &UiGlobalTransform), (With<MaterialCheckbox>, Without<TestId>)>,
    switches: Query<(Entity, &UiGlobalTransform), (With<MaterialSwitch>, Without<TestId>)>,
    radios: Query<(Entity, &UiGlobalTransform), (With<MaterialRadio>, Without<TestId>)>,
    sliders: Query<(Entity, &UiGlobalTransform), (With<MaterialSlider>, Without<TestId>)>,
    slider_tracks: Query<(Entity, &UiGlobalTransform), (With<SliderTrack>, Without<TestId>)>,
    slider_thumbs: Query<(Entity, &UiGlobalTransform), (With<SliderHandle>, Without<TestId>)>,
    text_fields: Query<(Entity, &UiGlobalTransform), (With<MaterialTextField>, Without<TestId>)>,
    selects: Query<(Entity, &UiGlobalTransform), (With<MaterialSelect>, Without<TestId>)>,
    select_options: Query<
        (Entity, &UiGlobalTransform),
        (
            With<bevy_material_ui::select::SelectOptionItem>,
            Without<TestId>,
        ),
    >,
) {
    if !telemetry.enabled {
        return;
    }

    match selected.current {
        ComponentSection::Checkboxes => {
            let mut items: Vec<(Entity, f32)> = checkboxes
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("checkbox_{}", i)),
                });
            }
        }
        ComponentSection::Switches => {
            let mut items: Vec<(Entity, f32)> =
                switches.iter().map(|(e, t)| (e, t.translation.y)).collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("switch_{}", i)),
                });
            }
        }
        ComponentSection::RadioButtons => {
            let mut items: Vec<(Entity, f32)> =
                radios.iter().map(|(e, t)| (e, t.translation.y)).collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("radio_{}", i)),
                });
            }
        }
        ComponentSection::Sliders => {
            // Slider root entities (for mapping slider_0_value, etc.)
            let mut items: Vec<(Entity, f32)> =
                sliders.iter().map(|(e, t)| (e, t.translation.y)).collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("slider_{}", i)),
                });
            }

            // Slider tracks (used by some tests for direct clicking)
            let mut tracks: Vec<(Entity, f32)> = slider_tracks
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            tracks.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in tracks.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("slider_track_{}", i)),
                });
            }

            // Slider thumbs (used as drag start points)
            let mut thumbs: Vec<(Entity, f32)> = slider_thumbs
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            thumbs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in thumbs.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("slider_thumb_{}", i)),
                });
            }
        }
        ComponentSection::TextFields => {
            let mut items: Vec<(Entity, f32)> = text_fields
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("text_field_{}", i)),
                });
            }
        }
        ComponentSection::Select => {
            let mut roots: Vec<(Entity, f32)> =
                selects.iter().map(|(e, t)| (e, t.translation.y)).collect();
            roots.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in roots.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("select_{}", i)),
                });
            }

            // Options are spawned when a select is opened; assign IDs when present.
            let mut opts: Vec<(Entity, f32)> = select_options
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            opts.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in opts.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("select_option_{}", i)),
                });
            }
        }
        _ => {}
    }
}

fn ensure_automation_test_ids_overlays_system(
    selected: Res<SelectedSection>,
    telemetry: Res<ComponentTelemetry>,
    mut commands: Commands,
    show_dialog_buttons: Query<
        (Entity, &UiGlobalTransform),
        (With<ShowDialogButton>, Without<TestId>),
    >,
    dialog_containers: Query<
        (Entity, &UiGlobalTransform),
        (With<DialogContainer>, Without<TestId>),
    >,
    dialog_close_buttons: Query<
        (Entity, &UiGlobalTransform),
        (With<DialogCloseButton>, Without<TestId>),
    >,
    dialog_confirm_buttons: Query<
        (Entity, &UiGlobalTransform),
        (With<DialogConfirmButton>, Without<TestId>),
    >,
    datetime_open_buttons: Query<
        (Entity, &UiGlobalTransform),
        (With<DateTimePickerOpenButton>, Without<TestId>),
    >,
    datetime_pickers: Query<
        (Entity, &UiGlobalTransform),
        (With<MaterialDateTimePicker>, Without<TestId>),
    >,
    menu_triggers: Query<(Entity, &UiGlobalTransform), (With<MenuTrigger>, Without<TestId>)>,
    menu_dropdowns: Query<(Entity, &UiGlobalTransform), (With<MenuDropdown>, Without<TestId>)>,
    menu_items: Query<(Entity, &UiGlobalTransform), (With<MenuItemMarker>, Without<TestId>)>,
    snackbar_triggers: Query<
        (Entity, &UiGlobalTransform),
        (With<SnackbarTrigger>, Without<TestId>),
    >,
    tooltip_demo_buttons: Query<
        (Entity, &UiGlobalTransform),
        (With<TooltipDemoButton>, Without<TestId>),
    >,
) {
    if !telemetry.enabled {
        return;
    }

    match selected.current {
        ComponentSection::Dialogs => {
            let mut opens: Vec<(Entity, f32)> = show_dialog_buttons
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            opens.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in opens.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("dialog_open_{}", i)),
                });
            }

            let mut containers: Vec<(Entity, f32)> = dialog_containers
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            containers.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in containers.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("dialog_container_{}", i)),
                });
            }

            let mut closes: Vec<(Entity, f32)> = dialog_close_buttons
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            closes.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in closes.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("dialog_close_{}", i)),
                });
            }

            let mut confirms: Vec<(Entity, f32)> = dialog_confirm_buttons
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            confirms.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in confirms.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("dialog_confirm_{}", i)),
                });
            }
        }
        ComponentSection::DateTimePicker => {
            let mut opens: Vec<(Entity, f32)> = datetime_open_buttons
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            opens.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in opens.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("datetime_open_{}", i)),
                });
            }

            let mut pickers: Vec<(Entity, f32)> = datetime_pickers
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            pickers.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in pickers.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("datetime_picker_{}", i)),
                });
            }
        }
        ComponentSection::Menus => {
            let mut triggers: Vec<(Entity, f32)> = menu_triggers
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            triggers.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in triggers.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("menu_trigger_{}", i)),
                });
            }

            let mut dropdowns: Vec<(Entity, f32)> = menu_dropdowns
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            dropdowns.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in dropdowns.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("menu_dropdown_{}", i)),
                });
            }

            let mut items: Vec<(Entity, f32)> = menu_items
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("menu_item_{}", i)),
                });
            }
        }
        ComponentSection::Snackbar => {
            let mut items: Vec<(Entity, f32)> = snackbar_triggers
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("snackbar_trigger_{}", i)),
                });
            }
        }
        ComponentSection::Tooltips => {
            let mut items: Vec<(Entity, f32)> = tooltip_demo_buttons
                .iter()
                .map(|(e, t)| (e, t.translation.y))
                .collect();
            items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for (i, (entity, _)) in items.into_iter().enumerate() {
                commands.queue(InsertTestIdIfExists {
                    entity,
                    test_id: TestId::new(format!("tooltip_demo_{}", i)),
                });
            }
        }
        _ => {}
    }
}

fn telemetry_from_component_events_system(
    mut checkbox_events: MessageReader<CheckboxChangeEvent>,
    mut switch_events: MessageReader<SwitchChangeEvent>,
    mut radio_events: MessageReader<RadioChangeEvent>,
    mut slider_events: MessageReader<SliderChangeEvent>,
    mut tab_events: MessageReader<TabChangeEvent>,
    slider_ids: Query<&TestId, With<MaterialSlider>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    if !telemetry.enabled {
        return;
    }

    for ev in checkbox_events.read() {
        telemetry.log_event(&format!("Checkbox changed: {:?}", ev.entity));
    }
    for ev in switch_events.read() {
        telemetry.log_event(&format!(
            "Switch changed: {:?} -> {}",
            ev.entity, ev.selected
        ));
    }
    for ev in radio_events.read() {
        telemetry.log_event(&format!("Radio changed: {:?}", ev.entity));
    }
    for ev in tab_events.read() {
        telemetry
            .states
            .insert("tab_selected".to_string(), ev.index.to_string());
        telemetry.log_event(&format!("Tab changed: {}", ev.index));
    }
    for ev in slider_events.read() {
        if let Ok(test_id) = slider_ids.get(ev.entity) {
            if let Some(idx) = test_id.id().strip_prefix("slider_") {
                telemetry
                    .states
                    .insert(format!("slider_{}_value", idx), format!("{:.2}", ev.value));
            }
        }
        telemetry.log_event(&format!(
            "Slider changed: {:?} -> {:.2}",
            ev.entity, ev.value
        ));
    }
}

fn telemetry_list_selection_state_system(
    selected: Res<SelectedSection>,
    items_changed: Query<(), (With<SelectableListItem>, Changed<MaterialListItem>)>,
    all_items: Query<(&TestId, &MaterialListItem), With<SelectableListItem>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    if !telemetry.enabled {
        return;
    }

    if selected.current != ComponentSection::Lists {
        return;
    }

    // Only recompute when something changed OR if the key is missing (first entry).
    let needs_update =
        !items_changed.is_empty() || !telemetry.states.contains_key("list_selected_items");
    if !needs_update {
        return;
    }

    let mut selected_ids: Vec<String> = Vec::new();
    for (test_id, item) in all_items.iter() {
        if item.selected {
            selected_ids.push(test_id.id().to_string());
        }
    }
    selected_ids.sort();

    let list_json = serde_json::to_string(&selected_ids).unwrap_or_else(|_| "[]".to_string());
    telemetry
        .states
        .insert("list_selected_items".to_string(), list_json);
    telemetry.states.insert(
        "list_selected_count".to_string(),
        selected_ids.len().to_string(),
    );
}

fn telemetry_snapshot_system(
    time: Res<Time>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    selected: Res<SelectedSection>,
    tabs: Query<&MaterialTabs>,
    nodes: Query<(
        &TestId,
        &ComputedNode,
        Option<&UiGlobalTransform>,
        &GlobalTransform,
    )>,
    mut telemetry: ResMut<ComponentTelemetry>,
    mut timer: Local<Timer>,
) {
    if !telemetry.enabled {
        return;
    }

    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(0.1, TimerMode::Repeating);
    }

    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    // Always keep selected section up-to-date in case it was set elsewhere.
    telemetry.states.insert(
        "selected_section".to_string(),
        selected.current.telemetry_name().to_string(),
    );

    if selected.current == ComponentSection::Tabs {
        if let Some(tabs) = tabs.iter().next() {
            telemetry
                .states
                .insert("tab_selected".to_string(), tabs.selected.to_string());
        }
    }

    let mut window_dims: Option<(f32, f32)> = None;
    if let Some(window) = windows.iter().next() {
        let w = window.resolution.physical_width() as f32;
        let h = window.resolution.physical_height() as f32;
        telemetry
            .states
            .insert("window_width".to_string(), w.to_string());
        telemetry
            .states
            .insert("window_height".to_string(), h.to_string());
        window_dims = Some((w, h));
    }

    telemetry.elements.clear();

    for (test_id, computed_node, ui_transform, global_transform) in nodes.iter() {
        let size = computed_node.size();
        if size.x <= 0.0 || size.y <= 0.0 {
            continue;
        }

        // Bevy UI uses physical pixels for `UiGlobalTransform`/`ComputedNode`.
        // Coordinates are in the window's client area.
        let center: Vec2 = if let Some(t) = ui_transform {
            t.translation
        } else {
            let t = global_transform.translation();
            Vec2::new(t.x, t.y)
        };
        let x = center.x - size.x / 2.0;
        let y = center.y - size.y / 2.0;

        telemetry.elements.insert(
            test_id.id().to_string(),
            ElementBounds {
                test_id: test_id.id().to_string(),
                x,
                y,
                width: size.x,
                height: size.y,
                parent: None,
            },
        );
    }

    // Ensure scaffold_* IDs are present for automation by synthesizing from known bounds.
    if let Some((w, h)) = window_dims {
        telemetry
            .elements
            .entry("scaffold_root".to_string())
            .or_insert(ElementBounds::new("scaffold_root", 0.0, 0.0, w, h));
    }

    if let Some(nav) = telemetry.elements.get("sidebar_scroll_container").cloned() {
        telemetry
            .elements
            .entry("scaffold_navigation".to_string())
            .or_insert(ElementBounds::new(
                "scaffold_navigation",
                nav.x,
                nav.y,
                nav.width,
                nav.height,
            ));
    }

    if let Some(content) = telemetry.elements.get("main_scroll_container").cloned() {
        telemetry
            .elements
            .entry("scaffold_content".to_string())
            .or_insert(ElementBounds::new(
                "scaffold_content",
                content.x,
                content.y,
                content.width,
                content.height,
            ));
    }

    let elements_with_bounds = telemetry.elements.len();
    telemetry.states.insert(
        "elements_with_bounds".to_string(),
        elements_with_bounds.to_string(),
    );
}

fn sidebar_scroll_telemetry_system(
    sidebar: Query<&ScrollPosition, With<SidebarNavScroll>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    if !telemetry.enabled {
        return;
    }

    let Some(pos) = sidebar.iter().next() else {
        return;
    };

    telemetry
        .states
        .insert("sidebar_scroll_y".to_string(), (**pos).y.to_string());
    telemetry
        .states
        .insert("sidebar_scroll_x".to_string(), (**pos).x.to_string());
}

fn main_scroll_telemetry_system(
    main: Query<&ScrollPosition, With<MainContentScroll>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    if !telemetry.enabled {
        return;
    }

    let Some(pos) = main.iter().next() else {
        return;
    };

    telemetry
        .states
        .insert("main_scroll_y".to_string(), (**pos).y.to_string());
    telemetry
        .states
        .insert("main_scroll_x".to_string(), (**pos).x.to_string());
}

fn progress_demo_animate_system(
    time: Res<Time>,
    mut bars: Query<(&mut MaterialLinearProgress, &mut ShowcaseProgressOscillator)>,
    mut labels: Query<&mut Text>,
) {
    for (mut progress, mut osc) in bars.iter_mut() {
        if progress.mode != ProgressMode::Determinate {
            continue;
        }

        let mut value = progress.progress + osc.direction * osc.speed * time.delta_secs();
        if value >= 1.0 {
            value = 1.0;
            osc.direction = -1.0;
        } else if value <= 0.0 {
            value = 0.0;
            osc.direction = 1.0;
        }

        progress.progress = value;

        if let Ok(mut text) = labels.get_mut(osc.label) {
            *text = Text::new(format!("{:>3}%", (value * 100.0).round() as i32));
        }
    }
}

fn argb_to_seed_color(argb: u32) -> Color {
    let r = ((argb >> 16) & 0xFF) as u8;
    let g = ((argb >> 8) & 0xFF) as u8;
    let b = (argb & 0xFF) as u8;
    Color::srgb_u8(r, g, b)
}

fn demo_click_log_system(
    mut icon_clicks: MessageReader<IconButtonClickEvent>,
    mut fab_clicks: MessageReader<FabClickEvent>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for ev in icon_clicks.read() {
        telemetry.log_event(&format!("IconButton clicked: {:?}", ev.entity));
    }
    for ev in fab_clicks.read() {
        telemetry.log_event(&format!("FAB clicked: {:?}", ev.entity));
    }
}

fn list_demo_mode_options_system(
    mut options: ResMut<ListDemoOptions>,
    mut mode_buttons: Query<(&ListSelectionModeOption, &Interaction), Changed<Interaction>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for (opt, interaction) in mode_buttons.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if options.mode != opt.0 {
            options.mode = opt.0;
            telemetry.log_event("List: selection mode changed");
        }
    }
}

fn list_demo_mode_style_system(
    theme: Res<MaterialTheme>,
    options: Res<ListDemoOptions>,
    mut chips: Query<(&ListSelectionModeOption, &mut MaterialChip)>,
) {
    if !theme.is_changed() && !options.is_changed() {
        return;
    }

    for (opt, mut chip) in chips.iter_mut() {
        chip.selected = opt.0 == options.mode;
    }
}

fn list_demo_apply_selection_mode_system(
    options: Res<ListDemoOptions>,
    lists_added: Query<(), Added<ListDemoRoot>>,
    mut lists: Query<(Entity, &mut bevy_material_ui::list::MaterialList), With<ListDemoRoot>>,
    children_query: Query<&Children>,
    mut items: Query<&mut bevy_material_ui::list::MaterialListItem>,
) {
    if !options.is_changed() && lists_added.is_empty() {
        return;
    }

    for (list_entity, mut list) in lists.iter_mut() {
        list.selection_mode = options.mode;

        // If switching to single-select, ensure at most one item is selected.
        if options.mode == bevy_material_ui::list::ListSelectionMode::Single {
            let mut kept_one = false;
            let mut stack: Vec<Entity> = vec![list_entity];
            while let Some(node) = stack.pop() {
                if let Ok(children) = children_query.get(node) {
                    for child in children.iter() {
                        if let Ok(mut item) = items.get_mut(child) {
                            if item.selected {
                                if kept_one {
                                    item.selected = false;
                                } else {
                                    kept_one = true;
                                }
                            }
                        }
                        stack.push(child);
                    }
                }
            }
        }
    }
}

fn setup_telemetry(mut telemetry: ResMut<ComponentTelemetry>) {
    telemetry.enabled = std::env::var("BEVY_TELEMETRY").is_ok();
    if telemetry.enabled {
        info!("📊 Telemetry enabled - writing to telemetry.json");
        telemetry.log_event("Showcase started");
    }
}

fn write_telemetry(telemetry: Res<ComponentTelemetry>) {
    if telemetry.is_changed() {
        telemetry.write_to_file();
    }
}

fn setup_ui(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    selection: Res<ShowcaseThemeSelection>,
    icon_font: Res<MaterialIconFont>,
    selected: Res<SelectedSection>,
    size_class: Res<WindowSizeClass>,
    mut materials: ResMut<Assets<ShapeMorphMaterial>>,
    tab_cache: Res<TabStateCache>,
) {
    // UI camera (renders over the 3d scene)
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
    ));

    let icon_font = icon_font.0.clone();

    // Global snackbar host overlay (required for ShowSnackbar events to display).
    commands.spawn(SnackbarHostBuilder::build());

    spawn_ui_root(
        &mut commands,
        &theme,
        selected.current,
        selection.seed_argb,
        icon_font,
        size_class.clone(),
        &mut materials,
        &tab_cache,
    );
}

fn spawn_ui_root(
    commands: &mut Commands,
    theme: &MaterialTheme,
    selected: ComponentSection,
    seed_argb: u32,
    icon_font: Handle<Font>,
    size_class: WindowSizeClass,
    materials: &mut Assets<ShapeMorphMaterial>,
    tab_cache: &TabStateCache,
) {
    commands
        .spawn((
            UiRoot,
            TestId::new("scaffold_root"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(theme.surface.with_alpha(0.0)),
        ))
        .with_children(|root| {
            let use_bottom_nav = !size_class.use_nav_rail();
            let nav_slot = move |sidebar: &mut ChildSpawnerCommands| {
                sidebar
                    .spawn((
                        TestId::new("scaffold_navigation"),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            // Important for scroll containers inside flex parents
                            min_height: Val::Px(0.0),
                            flex_grow: 1.0,
                            ..default()
                        },
                    ))
                    .with_children(|sidebar| {
                        if use_bottom_nav {
                            // Compact: bottom navigation surface. Use horizontal scrolling so all sections
                            // remain reachable and the automation can drag the horizontal thumb.
                            sidebar
                                .spawn((
                                    SidebarNavScroll,
                                    TestId::new("sidebar_scroll_container"),
                                    ScrollContainerBuilder::new().horizontal().build(),
                                    ScrollPosition::default(),
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        // Important for scroll containers inside flex parents
                                        min_width: Val::Px(0.0),
                                        flex_direction: FlexDirection::Row,
                                        column_gap: Val::Px(4.0),
                                        align_items: AlignItems::Center,
                                        overflow: Overflow::scroll_x(),
                                        ..default()
                                    },
                                ))
                                .with_children(|nav| {
                                    for section in ComponentSection::all() {
                                        spawn_nav_item_horizontal(
                                            nav,
                                            theme,
                                            *section,
                                            *section == selected,
                                        );
                                    }
                                    spawn_scrollbars(nav, theme, ScrollDirection::Horizontal);
                                });
                        } else {
                            sidebar.spawn((
                                Text::new("Material UI Showcase"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface),
                                Node {
                                    margin: UiRect::bottom(Val::Px(8.0)),
                                    ..default()
                                },
                            ));

                            // Scrollable navigation list (real MaterialList + ScrollContainer)
                            sidebar
                                .spawn(ListBuilder::new().build_scrollable())
                                .insert(SidebarNavScroll)
                                .insert(TestId::new("sidebar_scroll_container"))
                                .insert(Node {
                                    flex_grow: 1.0,
                                    width: Val::Percent(100.0),
                                    // Important for scroll containers inside flex columns
                                    min_height: Val::Px(0.0),
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(4.0),
                                    overflow: Overflow::scroll_y(),
                                    ..default()
                                })
                                .with_children(|nav| {
                                    for section in ComponentSection::all() {
                                        spawn_nav_item(nav, theme, *section, *section == selected);
                                    }
                                    spawn_scrollbars(nav, theme, ScrollDirection::Vertical);
                                });
                        }
                    });
            };

            let content_slot = |content: &mut ChildSpawnerCommands| {
                content
                    .spawn((
                        TestId::new("scaffold_content"),
                        Node {
                            width: Val::Percent(100.0),
                            // Important for scroll containers inside flex parents
                            min_height: Val::Px(0.0),
                            flex_direction: FlexDirection::Column,
                            flex_grow: 1.0,
                            ..default()
                        },
                    ))
                    .with_children(|content| {
                        content
                            .spawn((
                                DetailContent,
                                Node {
                                    width: Val::Percent(100.0),
                                    // Using `flex_grow` avoids percent-height inside an auto-sized
                                    // flex container (notably the bottom-nav scaffold), which can
                                    // otherwise resolve to 0 until a resize/layout rebuild.
                                    flex_grow: 1.0,
                                    // Important for scroll containers inside flex columns
                                    min_height: Val::Px(0.0),
                                    padding: UiRect::all(Val::Px(16.0)),
                                    overflow: Overflow::clip_y(),
                                    ..default()
                                },
                                BackgroundColor(theme.surface),
                            ))
                            .with_children(|detail| {
                                spawn_detail_scroller(
                                    detail, theme, selected, seed_argb, icon_font, materials,
                                    tab_cache,
                                );
                            });
                    });
            };

            // Keep the same nav/content TestIds so automation stays stable.
            let scaffold = AdaptiveNavigationScaffold {
                drawer: PermanentDrawerScaffold {
                    navigation_width_px: 240.0,
                    navigation_padding_px: 12.0,
                    content_padding_px: 0.0,
                    ..default()
                },
                rail: NavigationRailScaffold {
                    // Wider than a typical rail so our existing list-based navigation fits.
                    rail_width_px: 240.0,
                    rail_padding_px: 12.0,
                    content_padding_px: 0.0,
                    ..default()
                },
                bottom: BottomNavigationScaffold {
                    bottom_bar_height_px: 96.0,
                    bottom_bar_padding_px: 12.0,
                    content_padding_px: 0.0,
                    ..default()
                },
            };

            spawn_adaptive_navigation_scaffold(
                root,
                theme,
                &size_class,
                &scaffold,
                nav_slot,
                content_slot,
            );
        });
}

fn rebuild_ui_on_size_class_change_system(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    selected: Res<SelectedSection>,
    selection: Res<ShowcaseThemeSelection>,
    icon_font: Res<MaterialIconFont>,
    size_class: Res<WindowSizeClass>,
    mut gate: ResMut<LayoutRebuildGate>,
    mut materials: ResMut<Assets<ShapeMorphMaterial>>,
    tab_cache: Res<TabStateCache>,
    roots: Query<Entity, With<UiRoot>>,
    children_q: Query<&Children>,
) {
    if !gate.initialized {
        gate.initialized = true;
        gate.last_width = Some(size_class.width);
        gate.last_height = Some(size_class.height);
        return;
    }

    // `WindowSizeClass` also tracks raw pixel dimensions. During a window drag-resize,
    // those change continuously; rebuilding the entire UI tree on every update causes
    // visible flicker (content appears/disappears).
    let width_changed = gate.last_width != Some(size_class.width);
    let height_changed = gate.last_height != Some(size_class.height);
    if !width_changed && !height_changed {
        return;
    }

    gate.last_width = Some(size_class.width);
    gate.last_height = Some(size_class.height);

    for root in roots.iter() {
        clear_children_recursive(&mut commands, &children_q, root);
        commands.entity(root).despawn();
    }

    spawn_ui_root(
        &mut commands,
        &theme,
        selected.current,
        selection.seed_argb,
        icon_font.0.clone(),
        size_class.clone(),
        &mut materials,
        &tab_cache,
    );
}

fn spawn_detail_scroller(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    selected: ComponentSection,
    seed_argb: u32,
    icon_font: Handle<Font>,
    mut materials: &mut Assets<ShapeMorphMaterial>,
    tab_cache: &TabStateCache,
) {
    parent
        .spawn((
            MainContentScroll,
            TestId::new("main_scroll_container"),
            ScrollContainerBuilder::new().both().build(),
            ScrollPosition::default(),
            Node {
                flex_grow: 1.0,
                width: Val::Percent(100.0),
                // Important for scroll containers inside flex parents
                min_height: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                overflow: Overflow {
                    x: OverflowAxis::Scroll,
                    y: OverflowAxis::Scroll,
                },
                ..default()
            },
        ))
        .with_children(|scroller| {
            scroller
                .spawn((
                    DetailSurface,
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(16.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Stretch,
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_low),
                    BorderRadius::all(Val::Px(16.0)),
                ))
                .with_children(|surface| {
                    spawn_selected_section(
                        surface,
                        theme,
                        selected,
                        seed_argb,
                        icon_font,
                        &mut materials,
                        tab_cache,
                    );
                });

            // Optional: explicit call still ok; scroll plugin also ensures scrollbars.
            spawn_scrollbars(scroller, theme, ScrollDirection::Both);
        });
}

fn theme_mode_option_system(
    mut theme: ResMut<MaterialTheme>,
    selection: Res<ShowcaseThemeSelection>,
    mut options: Query<(&ThemeModeOption, &Interaction), Changed<Interaction>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for (opt, interaction) in options.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if theme.mode != opt.0 {
            *theme = MaterialTheme::from_seed(argb_to_seed_color(selection.seed_argb), opt.0);
            telemetry.log_event("Theme: mode changed");
        }
    }
}

fn theme_seed_option_system(
    mut theme: ResMut<MaterialTheme>,
    mut selection: ResMut<ShowcaseThemeSelection>,
    mut options: Query<(&ThemeSeedOption, &Interaction), Changed<Interaction>>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for (opt, interaction) in options.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if selection.seed_argb != opt.0 {
            selection.seed_argb = opt.0;
            *theme = MaterialTheme::from_seed(argb_to_seed_color(selection.seed_argb), theme.mode);
            telemetry.log_event("Theme: seed changed");
        }
    }
}

fn attach_theme_seed_text_field_system(
    mut commands: Commands,
    slots: Query<(Entity, &Children), With<ThemeSeedTextFieldSlot>>,
    children: Query<&Children>,
    fields: Query<(), With<MaterialTextField>>,
) {
    fn find_field_entity(
        root: Entity,
        children_q: &Query<&Children>,
        is_field: &Query<(), With<MaterialTextField>>,
    ) -> Option<Entity> {
        if is_field.get(root).is_ok() {
            return Some(root);
        }
        let Ok(kids) = children_q.get(root) else {
            return None;
        };
        for child in kids.iter() {
            if let Some(found) = find_field_entity(child, children_q, is_field) {
                return Some(found);
            }
        }
        None
    }

    for (slot_entity, slot_children) in slots.iter() {
        let mut field_entity: Option<Entity> = None;
        for child in slot_children.iter() {
            field_entity = find_field_entity(child, &children, &fields);
            if field_entity.is_some() {
                break;
            }
        }

        let Some(field_entity) = field_entity else {
            continue;
        };

        commands.entity(field_entity).insert(ThemeSeedTextField);
        commands
            .entity(slot_entity)
            .remove::<ThemeSeedTextFieldSlot>();
    }
}

fn theme_seed_text_field_system(
    mut theme: ResMut<MaterialTheme>,
    mut selection: ResMut<ShowcaseThemeSelection>,
    mut telemetry: ResMut<ComponentTelemetry>,
    mut change_events: MessageReader<TextFieldChangeEvent>,
    mut submit_events: MessageReader<TextFieldSubmitEvent>,
    tagged: Query<(), With<ThemeSeedTextField>>,
    mut fields: Query<&mut MaterialTextField>,
) {
    // Apply as soon as the input parses (ideal for paste).
    for ev in change_events.read() {
        if tagged.get(ev.entity).is_err() {
            continue;
        }

        let Some(argb) = parse_seed_hex_to_argb(&ev.value) else {
            continue;
        };

        if selection.seed_argb != argb {
            selection.seed_argb = argb;
            *theme = MaterialTheme::from_seed(argb_to_seed_color(selection.seed_argb), theme.mode);
            telemetry.log_event("Theme: seed changed (text)");
        }

        if let Ok(mut field) = fields.get_mut(ev.entity) {
            field.error = false;
            field.error_text = None;
        }
    }

    // On submit, show an error if it's still not parseable.
    for ev in submit_events.read() {
        if tagged.get(ev.entity).is_err() {
            continue;
        }

        let Ok(mut field) = fields.get_mut(ev.entity) else {
            continue;
        };

        match parse_seed_hex_to_argb(&ev.value) {
            Some(argb) => {
                if selection.seed_argb != argb {
                    selection.seed_argb = argb;
                    *theme = MaterialTheme::from_seed(
                        argb_to_seed_color(selection.seed_argb),
                        theme.mode,
                    );
                    telemetry.log_event("Theme: seed changed (text submit)");
                }
                field.error = false;
                field.error_text = None;
            }
            None => {
                field.error = true;
                field.error_text =
                    Some("Expected #RRGGBB, RRGGBB, 0xRRGGBB, or 0xFFRRGGBB".to_string());
            }
        }
    }
}

fn parse_seed_hex_to_argb(input: &str) -> Option<u32> {
    let mut s = input.trim();
    s = s.strip_prefix("Seed:").unwrap_or(s).trim();
    s = s.strip_prefix('#').unwrap_or(s);

    if let Some(rest) = s.strip_prefix("0x") {
        s = rest;
    } else if let Some(rest) = s.strip_prefix("0X") {
        s = rest;
    }

    let hex = s.trim();
    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
        }
        8 => {
            let a = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let r = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let g = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let b = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
        }
        _ => None,
    }
}

fn is_valid_email(value: &str) -> bool {
    let value = value.trim();
    if value.is_empty() {
        return true;
    }
    if value.contains(char::is_whitespace) {
        return false;
    }
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    // Very small, demo-oriented check: require at least one dot in the domain.
    domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

fn email_validation_system(
    mut changes: MessageReader<TextFieldChangeEvent>,
    mut fields: Query<&mut MaterialTextField>,
) {
    for ev in changes.read() {
        let Ok(mut field) = fields.get_mut(ev.entity) else {
            continue;
        };

        if field.input_type != InputType::Email {
            continue;
        }

        let valid = is_valid_email(&ev.value);
        if valid {
            field.error = false;
            field.error_text = None;
        } else {
            field.error = true;
            field.error_text = Some("Invalid email address".to_string());
        }
    }
}

#[allow(clippy::type_complexity)]
fn menu_demo_system(
    mut triggers: Query<(&ChildOf, &Interaction), (With<MenuTrigger>, Changed<Interaction>)>,
    mut dropdowns: Query<(&ChildOf, &mut Visibility), With<MenuDropdown>>,
    mut items: Query<(&ChildOf, &Interaction, &MenuItemMarker), Changed<Interaction>>,
    triggers_all: Query<(Entity, &ChildOf), With<MenuTrigger>>,
    mut selected_text: Query<(&ChildOf, &mut Text), With<MenuSelectedText>>,
    parents: Query<&ChildOf>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    // Build lookup: container -> trigger entity
    let mut trigger_by_container: std::collections::HashMap<Entity, Entity> =
        std::collections::HashMap::new();
    for (trigger_entity, parent) in triggers_all.iter() {
        trigger_by_container.insert(parent.0, trigger_entity);
    }

    // Toggle dropdown on trigger press
    for (parent, interaction) in triggers.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let container = parent.0;
        for (drop_parent, mut vis) in dropdowns.iter_mut() {
            if drop_parent.0 == container {
                *vis = match *vis {
                    Visibility::Hidden => Visibility::Inherited,
                    _ => Visibility::Hidden,
                };
            }
        }
    }

    // Select item
    for (parent, interaction, label) in items.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        // Item parent is the dropdown; dropdown parent is the container.
        let dropdown_entity = parent.0;
        let Ok(container_parent) = parents.get(dropdown_entity) else {
            continue;
        };
        let container = container_parent.0;

        // Update selected text on trigger button
        if let Some(trigger_entity) = trigger_by_container.get(&container).copied() {
            for (text_parent, mut text) in selected_text.iter_mut() {
                if text_parent.0 == trigger_entity {
                    *text = Text::new(label.0.as_str());
                }
            }
        }

        // Close dropdown
        for (drop_parent, mut vis) in dropdowns.iter_mut() {
            if drop_parent.0 == container {
                *vis = Visibility::Hidden;
            }
        }

        telemetry.log_event(&format!("Menu: selected {}", label.0));
    }
}

#[allow(clippy::type_complexity)]
fn datetime_picker_demo_system(
    mut open_buttons: Query<(&Interaction, &DateTimePickerOpenButton), Changed<Interaction>>,
    mut pickers: Query<&mut MaterialDateTimePicker>,
    mut submit: MessageReader<DateTimePickerSubmitEvent>,
    mut cancel: MessageReader<DateTimePickerCancelEvent>,
    mut result_texts: Query<(&DateTimePickerResultDisplay, &mut Text)>,
) {
    // Open picker when the demo button is pressed.
    for (interaction, open_button) in open_buttons.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Ok(mut picker) = pickers.get_mut(open_button.0) {
            picker.open = true;
        }
    }

    // Update result text on submit.
    for ev in submit.read() {
        let label = format!(
            "Result: {:04}-{:02}-{:02} {:02}:{:02}",
            ev.date.year, ev.date.month, ev.date.day, ev.hour, ev.minute
        );

        for (display, mut text) in result_texts.iter_mut() {
            if display.0 == ev.entity {
                *text = Text::new(label.as_str());
            }
        }
    }

    // Update result text on cancel.
    for ev in cancel.read() {
        for (display, mut text) in result_texts.iter_mut() {
            if display.0 == ev.entity {
                *text = Text::new("Result: Canceled");
            }
        }
    }
}

fn rebuild_ui_on_theme_change_system(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    selected: Res<SelectedSection>,
    selection: Res<ShowcaseThemeSelection>,
    icon_font: Res<MaterialIconFont>,
    mut gate: ResMut<ThemeRebuildGate>,
    mut materials: ResMut<Assets<ShapeMorphMaterial>>,
    tab_cache: Res<TabStateCache>,
    roots: Query<Entity, With<UiRoot>>,
    children_q: Query<&Children>,
    windows: Query<&Window>,
) {
    // `MaterialTheme` is inserted during app startup, which marks it as changed.
    // Skip the first tick to avoid rebuilding UI immediately (and causing double-despawn warnings).
    if !gate.initialized {
        gate.initialized = true;
        return;
    }

    if !theme.is_changed() {
        return;
    }

    for root in roots.iter() {
        clear_children_recursive(&mut commands, &children_q, root);
        commands.entity(root).despawn();
    }

    let window_size = windows
        .iter()
        .next()
        .map(|w| (w.width(), w.height()))
        .unwrap_or((1280.0, 720.0));

    spawn_ui_root(
        &mut commands,
        &theme,
        selected.current,
        selection.seed_argb,
        icon_font.0.clone(),
        WindowSizeClass::new(window_size.0, window_size.1),
        &mut materials,
        &tab_cache,
    );
}

/// System to cache tab states before UI rebuild
fn cache_tab_state_system(
    mut tab_cache: ResMut<TabStateCache>,
    selected: Res<SelectedSection>,
    tabs: Query<&MaterialTabs>,
) {
    // Only cache if we're on the Tabs section
    if selected.current == ComponentSection::Tabs {
        if let Some(tabs_component) = tabs.iter().next() {
            tab_cache
                .selections
                .insert(ComponentSection::Tabs, tabs_component.selected);
        }
    }
}

fn snackbar_demo_options_system(
    mut options: ResMut<SnackbarDemoOptions>,
    mut duration_buttons: Query<(&SnackbarDurationOption, &Interaction), Changed<Interaction>>,
    mut action_toggle: Query<&Interaction, (Changed<Interaction>, With<SnackbarActionToggle>)>,
) {
    for (opt, interaction) in duration_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.duration = opt.0;
        }
    }

    for interaction in action_toggle.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.has_action = !options.has_action;
        }
    }
}

fn snackbar_demo_trigger_system(
    options: Res<SnackbarDemoOptions>,
    mut triggers: Query<&Interaction, (Changed<Interaction>, With<SnackbarTrigger>)>,
    mut show: MessageWriter<ShowSnackbar>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for interaction in triggers.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let mut evt = if options.has_action {
            ShowSnackbar::with_action("Item deleted", "UNDO")
        } else {
            ShowSnackbar::message("Item deleted")
        };

        evt.duration = Some(options.duration);
        show.write(evt);
        telemetry.log_event("Snackbar: show");
    }
}

fn snackbar_demo_style_system(
    theme: Res<MaterialTheme>,
    options: Res<SnackbarDemoOptions>,
    mut duration_chips: Query<
        (&SnackbarDurationOption, &mut MaterialChip),
        Without<SnackbarActionToggle>,
    >,
    mut action_toggle_chip: Query<
        &mut MaterialChip,
        (With<SnackbarActionToggle>, Without<SnackbarDurationOption>),
    >,
) {
    if !theme.is_changed() && !options.is_changed() {
        return;
    }

    for (opt, mut chip) in duration_chips.iter_mut() {
        chip.selected = (opt.0 - options.duration).abs() < 0.01;
    }

    for mut chip in action_toggle_chip.iter_mut() {
        chip.selected = options.has_action;
    }
}

fn snackbar_demo_action_log_system(
    mut actions: MessageReader<SnackbarActionEvent>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    for ev in actions.read() {
        telemetry.log_event(&format!("Snackbar action: {}", ev.action));
    }
}

fn tooltip_demo_options_system(
    mut options: ResMut<TooltipDemoOptions>,
    mut position_buttons: Query<(&TooltipPositionOption, &Interaction), Changed<Interaction>>,
    mut delay_buttons: Query<(&TooltipDelayOption, &Interaction), Changed<Interaction>>,
) {
    for (opt, interaction) in position_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.position = opt.0;
        }
    }

    for (opt, interaction) in delay_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.delay = opt.0;
        }
    }
}

fn tooltip_demo_apply_system(
    options: Res<TooltipDemoOptions>,
    mut triggers: Query<&mut TooltipTrigger, With<TooltipDemoButton>>,
    mut tooltips: Query<&mut Tooltip>,
    mut telemetry: ResMut<ComponentTelemetry>,
) {
    if !options.is_changed() {
        return;
    }

    for mut trigger in triggers.iter_mut() {
        trigger.position = options.position;
        trigger.delay = options.delay;

        // If a tooltip is currently visible, update its position immediately.
        if let Some(tooltip_entity) = trigger.tooltip_entity {
            if let Ok(mut tooltip) = tooltips.get_mut(tooltip_entity) {
                tooltip.position = options.position;
            }
        }
    }

    telemetry.log_event("Tooltip: options changed");
}

fn tooltip_demo_style_system(
    theme: Res<MaterialTheme>,
    options: Res<TooltipDemoOptions>,
    mut position_buttons: Query<
        (
            Entity,
            &TooltipPositionOption,
            &mut MaterialButton,
            &Children,
        ),
        Without<TooltipDelayOption>,
    >,
    mut delay_buttons: Query<
        (Entity, &TooltipDelayOption, &mut MaterialButton, &Children),
        Without<TooltipPositionOption>,
    >,
    mut label_colors: Query<&mut TextColor, With<ButtonLabel>>,
) {
    if !theme.is_changed() && !options.is_changed() {
        return;
    }

    for (_entity, opt, mut button, children) in position_buttons.iter_mut() {
        let selected = opt.0 == options.position;
        button.variant = if selected {
            ButtonVariant::FilledTonal
        } else {
            ButtonVariant::Outlined
        };

        let text_color = button.text_color(&theme);
        for child in children.iter() {
            if let Ok(mut color) = label_colors.get_mut(child) {
                *color = TextColor(text_color);
            }
        }
    }

    for (_entity, opt, mut button, children) in delay_buttons.iter_mut() {
        let selected = (opt.0 - options.delay).abs() < 0.01;
        button.variant = if selected {
            ButtonVariant::FilledTonal
        } else {
            ButtonVariant::Outlined
        };

        let text_color = button.text_color(&theme);
        for child in children.iter() {
            if let Ok(mut color) = label_colors.get_mut(child) {
                *color = TextColor(text_color);
            }
        }
    }
}

fn dialog_demo_position_options_system(
    mut options: ResMut<DialogDemoOptions>,
    mut position_buttons: Query<(&DialogPositionOption, &Interaction), Changed<Interaction>>,
) {
    for (opt, interaction) in position_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.position = opt.0;
        }
    }
}

fn dialog_demo_modal_options_system(
    mut options: ResMut<DialogDemoOptions>,
    mut modal_buttons: Query<(&DialogModalOption, &Interaction), Changed<Interaction>>,
) {
    for (opt, interaction) in modal_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            options.modal = opt.0;
        }
    }
}

fn dialog_demo_position_style_system(
    theme: Res<MaterialTheme>,
    options: Res<DialogDemoOptions>,
    mut position_chips: Query<(&DialogPositionOption, &mut MaterialChip)>,
) {
    if !theme.is_changed() && !options.is_changed() {
        return;
    }

    for (opt, mut chip) in position_chips.iter_mut() {
        chip.selected = opt.0 == options.position;
    }
}

fn dialog_demo_modal_style_system(
    theme: Res<MaterialTheme>,
    options: Res<DialogDemoOptions>,
    mut modal_chips: Query<(&DialogModalOption, &mut MaterialChip)>,
) {
    if !theme.is_changed() && !options.is_changed() {
        return;
    }

    for (opt, mut chip) in modal_chips.iter_mut() {
        chip.selected = opt.0 == options.modal;
    }
}

fn dialog_demo_apply_position_system(
    options: Res<DialogDemoOptions>,
    dialogs_added: Query<(), Added<DialogContainer>>,
    mut dialogs: Query<&mut Node, With<DialogContainer>>,
) {
    if !options.is_changed() && dialogs_added.is_empty() {
        return;
    }

    for mut node in dialogs.iter_mut() {
        match options.position {
            DialogPosition::CenterParent => {
                node.position_type = PositionType::Relative;
                node.left = Val::Auto;
                node.top = Val::Auto;
                node.right = Val::Auto;
                node.bottom = Val::Auto;
                node.align_self = AlignSelf::Center;
                node.margin = UiRect::vertical(Val::Px(8.0));
            }
            DialogPosition::BelowTrigger => {
                node.position_type = PositionType::Relative;
                node.left = Val::Auto;
                node.top = Val::Auto;
                node.right = Val::Auto;
                node.bottom = Val::Auto;
                node.align_self = AlignSelf::Start;
                node.margin = UiRect::top(Val::Px(12.0));
            }
            DialogPosition::CenterWindow => {
                // Approximate center by anchoring the dialog's top-left near center.
                // (UI centering with translation isn't directly available here.)
                node.position_type = PositionType::Absolute;
                node.left = Val::Percent(50.0);
                node.top = Val::Percent(50.0);
                node.right = Val::Auto;
                node.bottom = Val::Auto;
                node.align_self = AlignSelf::Auto;
                // Dialog width is fixed at 280px in the view; offset half width to better center.
                node.margin = UiRect {
                    left: Val::Px(-140.0),
                    top: Val::Px(-100.0),
                    ..default()
                };
            }
        }
    }
}

fn dialog_demo_apply_modal_system(
    options: Res<DialogDemoOptions>,
    dialogs_added: Query<(), Added<DialogContainer>>,
    mut dialogs: Query<&mut MaterialDialog, With<DialogContainer>>,
) {
    if !options.is_changed() && dialogs_added.is_empty() {
        return;
    }

    for mut dialog in dialogs.iter_mut() {
        dialog.modal = options.modal;
    }
}

fn dialog_demo_open_close_system(
    mut show_buttons: Query<&Interaction, (Changed<Interaction>, With<ShowDialogButton>)>,
    mut close_buttons: Query<&Interaction, (Changed<Interaction>, With<DialogCloseButton>)>,
    mut confirm_buttons: Query<&Interaction, (Changed<Interaction>, With<DialogConfirmButton>)>,
    mut dialog: Query<&mut MaterialDialog, With<DialogContainer>>,
    mut result_text: Query<&mut Text, With<DialogResultDisplay>>,
) {
    let mut open = false;
    let mut close_reason: Option<&'static str> = None;

    for interaction in show_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            open = true;
        }
    }

    for interaction in close_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            close_reason = Some("Cancelled");
        }
    }

    for interaction in confirm_buttons.iter_mut() {
        if *interaction == Interaction::Pressed {
            close_reason = Some("Confirmed");
        }
    }

    if open {
        for mut d in dialog.iter_mut() {
            d.open = true;
        }
    }

    if let Some(reason) = close_reason {
        for mut d in dialog.iter_mut() {
            d.open = false;
        }
        for mut text in result_text.iter_mut() {
            text.0 = format!("Result: {}", reason);
        }
    }
}

fn update_detail_content(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    selected: Res<SelectedSection>,
    selection: Res<ShowcaseThemeSelection>,
    icon_font: Res<MaterialIconFont>,
    mut materials: ResMut<Assets<ShapeMorphMaterial>>,
    tab_cache: Res<TabStateCache>,
    detail: Query<Entity, With<DetailContent>>,
    children_q: Query<&Children>,
) {
    if !selected.is_changed() {
        return;
    }

    let Some(detail_entity) = detail.iter().next() else {
        return;
    };

    clear_children_recursive(&mut commands, &children_q, detail_entity);

    let section = selected.current;
    let icon_font = icon_font.0.clone();
    commands.entity(detail_entity).with_children(|detail| {
        spawn_detail_scroller(
            detail,
            &theme,
            section,
            selection.seed_argb,
            icon_font,
            &mut materials,
            &tab_cache,
        );
    });
}

fn spawn_selected_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    section: ComponentSection,
    seed_argb: u32,
    icon_font: Handle<Font>,
    materials: &mut Assets<ShapeMorphMaterial>,
    tab_cache: &TabStateCache,
) {
    match section {
        ComponentSection::Buttons => spawn_buttons_section(parent, theme),
        ComponentSection::Checkboxes => spawn_checkboxes_section(parent, theme, Some(icon_font)),
        ComponentSection::Switches => spawn_switches_section(parent, theme),
        ComponentSection::RadioButtons => spawn_radios_section(parent, theme),
        ComponentSection::Chips => spawn_chips_section(parent, theme, icon_font),
        ComponentSection::Fab => spawn_fab_section(parent, theme, icon_font),
        ComponentSection::Badges => spawn_badges_section(parent, theme, icon_font),
        ComponentSection::Progress => spawn_progress_section(parent, theme),
        ComponentSection::Cards => spawn_cards_section(parent, theme),
        ComponentSection::Dividers => spawn_dividers_section(parent, theme),
        ComponentSection::Lists => spawn_list_section(parent, theme, icon_font),
        ComponentSection::Icons => spawn_icons_section(parent, theme, icon_font),
        ComponentSection::IconButtons => spawn_icon_buttons_section(parent, theme, icon_font),
        ComponentSection::Sliders => spawn_sliders_section(parent, theme),
        ComponentSection::TextFields => spawn_text_fields_section(parent, theme),
        ComponentSection::Dialogs => spawn_dialogs_section(parent, theme),
        ComponentSection::DateTimePicker => spawn_datetime_picker_section(parent, theme),
        ComponentSection::Menus => spawn_menus_section(parent, theme, icon_font),
        ComponentSection::Tabs => spawn_tabs_section(parent, theme, tab_cache),
        ComponentSection::Select => spawn_select_section(parent, theme, icon_font),
        ComponentSection::Snackbar => spawn_snackbar_section(parent, theme, icon_font),
        ComponentSection::Tooltips => spawn_tooltip_section(parent, theme, icon_font),
        ComponentSection::AppBar => spawn_app_bar_section(parent, theme, icon_font),
        ComponentSection::Toolbar => spawn_toolbar_section(parent, theme, icon_font),
        ComponentSection::Layouts => spawn_layouts_section(parent, theme, icon_font),
        ComponentSection::LoadingIndicator => {
            spawn_loading_indicator_section(parent, theme, materials)
        }
        ComponentSection::Search => spawn_search_section(parent, theme),
        ComponentSection::ThemeColors => spawn_theme_section(parent, theme, seed_argb),
    }
}

fn clear_children_recursive(
    commands: &mut Commands,
    children_q: &Query<&Children>,
    entity: Entity,
) {
    let Ok(children) = children_q.get(entity) else {
        return;
    };

    for child in children.iter() {
        clear_children_recursive(commands, children_q, child);
        commands.entity(child).despawn();
    }
}

fn rotate_dice(time: Res<Time>, mut dice: Query<&mut Transform, With<SpinningDice>>) {
    for mut transform in dice.iter_mut() {
        transform.rotate_y(time.delta_secs() * 0.8);
        transform.rotate_x(time.delta_secs() * 0.4);
    }
}

fn setup_3d_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.05, 0.05, 0.08)),
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 2500.0,
            ..default()
        },
        Transform::from_xyz(2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let mesh = meshes.add(create_d10_mesh());
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.75, 0.22, 0.28),
        metallic: 0.2,
        perceptual_roughness: 0.35,
        ..default()
    });

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        SpinningDice,
    ));
}

fn create_d10_mesh() -> Mesh {
    use std::f32::consts::PI;

    // A D10 is a pentagonal trapezohedron.
    let n: usize = 5;
    let top_height: f32 = 1.2;
    let bottom_height: f32 = -1.2;
    let mid_top: f32 = 0.35;
    let mid_bottom: f32 = -0.35;
    let top_radius: f32 = 0.9;
    let bottom_radius: f32 = 0.9;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let top_point = [0.0, top_height, 0.0];
    let bottom_point = [0.0, bottom_height, 0.0];

    let mut upper_ring: Vec<[f32; 3]> = Vec::with_capacity(n);
    for i in 0..n {
        let angle = (i as f32) * 2.0 * PI / (n as f32);
        upper_ring.push([top_radius * angle.cos(), mid_top, top_radius * angle.sin()]);
    }

    let mut lower_ring: Vec<[f32; 3]> = Vec::with_capacity(n);
    for i in 0..n {
        let angle = ((i as f32) + 0.5) * 2.0 * PI / (n as f32);
        lower_ring.push([
            bottom_radius * angle.cos(),
            mid_bottom,
            bottom_radius * angle.sin(),
        ]);
    }

    for i in 0..n {
        let next_i = (i + 1) % n;
        let prev_i = (i + n - 1) % n;

        add_triangle(
            &mut positions,
            &mut normals,
            &mut indices,
            top_point,
            upper_ring[i],
            lower_ring[i],
        );
        add_triangle(
            &mut positions,
            &mut normals,
            &mut indices,
            top_point,
            lower_ring[i],
            upper_ring[next_i],
        );

        add_triangle(
            &mut positions,
            &mut normals,
            &mut indices,
            bottom_point,
            lower_ring[i],
            upper_ring[i],
        );
        add_triangle(
            &mut positions,
            &mut normals,
            &mut indices,
            bottom_point,
            upper_ring[i],
            lower_ring[prev_i],
        );
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(indices))
}

fn add_triangle(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
) {
    let start = positions.len() as u32;
    positions.push(a);
    positions.push(b);
    positions.push(c);

    let ab = Vec3::from_array(b) - Vec3::from_array(a);
    let ac = Vec3::from_array(c) - Vec3::from_array(a);
    let n = ab.cross(ac).normalize_or_zero().to_array();

    normals.push(n);
    normals.push(n);
    normals.push(n);

    indices.push(start);
    indices.push(start + 1);
    indices.push(start + 2);
}
