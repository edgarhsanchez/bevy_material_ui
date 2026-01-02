pub mod common;
pub mod i18n_helpers;
pub mod navigation;
pub mod views;

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::loading_indicator::ShapeMorphMaterial;

use common::*;
use navigation::*;
use views::*;

#[derive(Component)]
struct SpinningDice;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .init_resource::<SelectedSection>()
        .init_resource::<ComponentTelemetry>()
        .add_systems(Startup, (setup_3d_scene, setup_ui, setup_telemetry))
        .add_systems(Update, (rotate_dice, handle_nav_clicks, update_nav_highlights, update_detail_content, write_telemetry))
        .run();
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
    icon_font: Res<MaterialIconFont>,
    selected: Res<SelectedSection>,
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

    // Root layout: sidebar + detail
    commands
        .spawn((
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
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(theme.on_surface),
                    Node { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
                ));

                sidebar
                    .spawn((
                        SidebarScrollArea,
                        ScrollContainerBuilder::new().vertical().with_scrollbars(true).build(),
                        ScrollPosition::default(),
                        Node {
                            flex_grow: 1.0,
                            // Important for scroll containers inside a flex column:
                            // allow shrinking so overflow/scrolling can happen.
                            min_height: Val::Px(0.0),
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.0),
                            overflow: Overflow::scroll(),
                            ..default()
                        },
                    ))
                    .with_children(|nav| {
                        for section in ComponentSection::all() {
                            spawn_nav_item(nav, &theme, *section, *section == selected.current);
                        }

                        // Scrollbars spawn automatically via ScrollPlugin's ensure_scrollbars_system
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
                BackgroundColor(theme.surface.with_alpha(0.0)),
            ))
            .with_children(|detail| {
                spawn_selected_section(detail, &theme, selected.current, icon_font.clone(), &mut materials, &tab_cache);
            });
        });
}

fn update_detail_content(
    mut commands: Commands,
    theme: Res<MaterialTheme>,
    selected: Res<SelectedSection>,
    icon_font: Res<MaterialIconFont>,
    mut materials: ResMut<Assets<ShapeMorphMaterial>>,
    tab_cache: Res<TabStateCache>,
    detail: Query<Entity, With<DetailContent>>,
    children_q: Query<&Children>,
) {
    if !selected.is_changed() {
        return;
    }

    let Ok(detail_entity) = detail.get_single() else {
        return;
    };

    clear_children_recursive(&mut commands, &children_q, detail_entity);

    let section = selected.current;
    let icon_font = icon_font.0.clone();
    commands.entity(detail_entity).with_children(|detail| {
        spawn_selected_section(detail, &theme, section, icon_font, &mut materials, &tab_cache);
    });
}

fn spawn_selected_section(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    section: ComponentSection,
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
        ComponentSection::Menus => spawn_menus_section(parent, theme, icon_font),
        ComponentSection::Tabs => spawn_tabs_section(parent, theme, tab_cache),
        ComponentSection::Select => spawn_select_section(parent, theme, icon_font),
        ComponentSection::Snackbar => spawn_snackbar_section(parent, theme, icon_font),
        ComponentSection::Tooltips => spawn_tooltip_section(parent, theme, icon_font),
        ComponentSection::AppBar => spawn_app_bar_section(parent, theme, icon_font),
        ComponentSection::Toolbar => spawn_toolbar_section(parent, theme, icon_font),
        ComponentSection::Layouts => spawn_layouts_section(parent, theme, icon_font),
        ComponentSection::LoadingIndicator => spawn_loading_indicator_section(parent, theme, materials),
        ComponentSection::Search => spawn_search_section(parent, theme),
        ComponentSection::ThemeColors => spawn_theme_section(parent, theme),
    }
}

fn clear_children_recursive(commands: &mut Commands, children_q: &Query<&Children>, entity: Entity) {
    let Ok(children) = children_q.get(entity) else {
        return;
    };

    for child in children.iter() {
        clear_children_recursive(commands, children_q, *child);
        commands.entity(*child).despawn();
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

        add_triangle(&mut positions, &mut normals, &mut indices, top_point, upper_ring[i], lower_ring[i]);
        add_triangle(&mut positions, &mut normals, &mut indices, top_point, lower_ring[i], upper_ring[next_i]);

        add_triangle(&mut positions, &mut normals, &mut indices, bottom_point, lower_ring[i], upper_ring[i]);
        add_triangle(&mut positions, &mut normals, &mut indices, bottom_point, upper_ring[i], lower_ring[prev_i]);
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

