#[path = "common.rs"]
pub mod common;

#[path = "navigation.rs"]
pub mod navigation;

#[path = "views/mod.rs"]
pub mod views;

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::text_field::InputType;
use bevy_material_ui::theme::ThemeMode;

use common::*;
use navigation::*;
use views::*;

#[derive(Resource, Clone)]
struct IconFont(Handle<Font>);

#[derive(Component)]
struct SpinningDice;

#[derive(Component)]
struct UiRoot;

#[derive(Resource, Default)]
struct ThemeRebuildGate {
    initialized: bool,
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

#[derive(Resource, Default)]
struct DialogDemoOptions {
    position: DialogPosition,
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
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
        .init_resource::<ThemeRebuildGate>()
        .add_systems(Startup, (setup_3d_scene, setup_ui, setup_telemetry))
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
                email_validation_system,
            ),
        )
        .add_systems(
            Update,
            (
                dialog_demo_position_options_system,
                dialog_demo_position_style_system,
                dialog_demo_apply_position_system,
                dialog_demo_open_close_system,
                list_demo_mode_options_system,
                list_demo_mode_style_system,
                list_demo_apply_selection_mode_system,
                theme_mode_option_system,
                theme_seed_option_system,
                rebuild_ui_on_theme_change_system,
                write_telemetry,
            ),
        )
        .run();
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
    asset_server: Res<AssetServer>,
    theme: Res<MaterialTheme>,
    selected: Res<SelectedSection>,
) {
    // UI camera (renders over the 3d scene)
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
    ));

    let icon_font = asset_server.load::<Font>("fonts/MaterialSymbolsOutlined.ttf");
    commands.insert_resource(IconFont(icon_font.clone()));

    // Global snackbar host overlay (required for ShowSnackbar events to display).
    commands.spawn(SnackbarHostBuilder::build());

    spawn_ui_root(&mut commands, &theme, selected.current, icon_font);
}

fn spawn_ui_root(
    commands: &mut Commands,
    theme: &MaterialTheme,
    selected: ComponentSection,
    icon_font: Handle<Font>,
) {
    // Root layout: sidebar + detail
    commands
        .spawn((
            UiRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(theme.surface.with_alpha(0.0)),
        ))
        .with_children(|root| {
            // Sidebar
            root.spawn((
                Node {
                    width: Val::Px(240.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(12.0)),
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(theme.surface_container_low),
            ))
            .with_children(|sidebar| {
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
            });

            // Detail content area
            root.spawn((
                DetailContent,
                Node {
                    flex_grow: 1.0,
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                BackgroundColor(theme.surface),
            ))
            .with_children(|detail| {
                detail
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            padding: UiRect::all(Val::Px(16.0)),
                            ..default()
                        },
                        BackgroundColor(theme.surface_container_low),
                        BorderRadius::all(Val::Px(16.0)),
                    ))
                    .with_children(|surface| {
                        spawn_selected_section(surface, theme, selected, icon_font);
                    });
            });
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
    icon_font: Res<IconFont>,
    mut gate: ResMut<ThemeRebuildGate>,
    roots: Query<Entity, With<UiRoot>>,
    children_q: Query<&Children>,
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

    spawn_ui_root(
        &mut commands,
        &theme,
        selected.current,
        icon_font.0.clone(),
    );
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
    mut duration_chips: Query<(&SnackbarDurationOption, &mut MaterialChip), Without<SnackbarActionToggle>>,
    mut action_toggle_chip: Query<&mut MaterialChip, (With<SnackbarActionToggle>, Without<SnackbarDurationOption>)>,
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
    mut position_buttons: Query<(Entity, &TooltipPositionOption, &mut MaterialButton, &Children), Without<TooltipDelayOption>>,
    mut delay_buttons: Query<(Entity, &TooltipDelayOption, &mut MaterialButton, &Children), Without<TooltipPositionOption>>,
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

fn dialog_demo_open_close_system(
    mut show_buttons: Query<&Interaction, (Changed<Interaction>, With<ShowDialogButton>)>,
    mut close_buttons: Query<&Interaction, (Changed<Interaction>, With<DialogCloseButton>)>,
    mut confirm_buttons: Query<&Interaction, (Changed<Interaction>, With<DialogConfirmButton>)>,
    mut dialog: Query<&mut Visibility, With<DialogContainer>>,
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
        for mut vis in dialog.iter_mut() {
            *vis = Visibility::Visible;
        }
    }

    if let Some(reason) = close_reason {
        for mut vis in dialog.iter_mut() {
            *vis = Visibility::Hidden;
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
    icon_font: Res<IconFont>,
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
        spawn_selected_section(detail, &theme, section, icon_font);
    });
}

fn spawn_selected_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    section: ComponentSection,
    icon_font: Handle<Font>,
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
        ComponentSection::Tabs => spawn_tabs_section(parent, theme),
        ComponentSection::Select => spawn_select_section(parent, theme, icon_font),
        ComponentSection::Snackbar => spawn_snackbar_section(parent, theme),
        ComponentSection::Tooltips => spawn_tooltip_section(parent, theme, icon_font),
        ComponentSection::AppBar => spawn_app_bar_section(parent, theme, icon_font),
        ComponentSection::ThemeColors => spawn_theme_section(parent, theme),
    }
}

fn clear_children_recursive(commands: &mut Commands, children_q: &Query<&Children>, entity: Entity) {
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

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD)
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
