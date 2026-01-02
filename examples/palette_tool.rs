use bevy::prelude::*;
use bevy_material_ui::prelude::*;
use bevy_material_ui::theme::ThemeMode;

use std::io::Write;
use std::process::{Command, Stdio};

const TONES: &[u8] = &[0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99, 100];

#[derive(Resource, Clone, Copy, PartialEq, Eq)]
struct PaletteToolState {
    r: u8,
    g: u8,
    b: u8,
    mode: ThemeMode,
}

impl Default for PaletteToolState {
    fn default() -> Self {
        Self {
            r: 0x67,
            g: 0x50,
            b: 0xA4,
            mode: ThemeMode::Dark,
        }
    }
}

#[derive(Component, Clone, Copy)]
enum SeedChannel {
    R,
    G,
    B,
}

#[derive(Component, Clone, Copy)]
struct SeedSliderSlot {
    channel: SeedChannel,
}

#[derive(Component, Clone, Copy)]
struct ChannelValueText {
    channel: SeedChannel,
}

#[derive(Component)]
struct SeedHexField;

#[derive(Component)]
struct SeedHexFieldSlot;

#[derive(Component)]
struct CopySeedButton;

#[derive(Resource, Default)]
struct SeedHexDraftState {
    /// True if the user has typed into the seed field since the last successful apply.
    dirty: bool,
}

#[derive(Component)]
struct SeedSwatch;

#[derive(Component, Clone, Copy)]
enum PaletteKind {
    Primary,
    Secondary,
    Tertiary,
    Neutral,
    NeutralVariant,
}

#[derive(Component, Clone, Copy)]
struct PaletteSwatch {
    palette: PaletteKind,
    tone: u8,
}

#[derive(Component, Clone, Copy)]
enum SchemeRole {
    Primary,
    OnPrimary,
    PrimaryContainer,
    OnPrimaryContainer,
    Secondary,
    OnSecondary,
    SecondaryContainer,
    OnSecondaryContainer,
    Tertiary,
    OnTertiary,
    TertiaryContainer,
    OnTertiaryContainer,
    Surface,
    OnSurface,
    OnSurfaceVariant,
    Outline,
    Error,
    OnError,
}

#[derive(Component, Clone, Copy)]
struct SchemeSwatch {
    role: SchemeRole,
}

#[derive(Component, Clone, Copy)]
struct SchemeHexText {
    role: SchemeRole,
}

fn main() {
    let state = PaletteToolState::default();
    App::new()
        .insert_resource(state)
        .insert_resource(SeedHexDraftState::default())
        .insert_resource(MaterialTheme::from_seed(
            Color::srgb_u8(state.r, state.g, state.b),
            state.mode,
        ))
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialUiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, attach_seed_channels_to_sliders_system)
        .add_systems(Update, attach_seed_hex_field_system)
        .add_systems(
            Update,
            (
                handle_slider_changes_system,
                handle_seed_hex_change_system,
                handle_seed_hex_submit_system,
                handle_mode_toggle_system,
                apply_state_to_theme_system,
                refresh_palette_preview_system,
                sync_controls_from_state_system,
                handle_copy_seed_button_system,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, theme: Res<MaterialTheme>, state: Res<PaletteToolState>) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(theme.surface),
        ))
        .with_children(|root| {
            // Controls panel
            root.spawn((
                Node {
                    width: Val::Px(360.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(16.0)),
                    row_gap: Val::Px(12.0),
                    ..default()
                },
                BackgroundColor(theme.surface_container),
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new("MD3 Palette Tool"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(theme.on_surface),
                ));

                // Seed preview row
                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            SeedSwatch,
                            Node {
                                width: Val::Px(36.0),
                                height: Val::Px(36.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb_u8(state.r, state.g, state.b)),
                            BorderRadius::all(Val::Px(8.0)),
                        ));

                        row.spawn(Node {
                            flex_grow: 1.0,
                            ..default()
                        })
                        .with_children(|container| {
                            // Wrap in a slot so we can tag the actual MaterialTextField entity
                            // after it's spawned by the helper.
                            container
                                .spawn((
                                    SeedHexFieldSlot,
                                    Node {
                                        width: Val::Percent(100.0),
                                        ..default()
                                    },
                                ))
                                .with_children(|slot| {
                                    slot.spawn_text_field_with(
                                        &theme,
                                        TextFieldBuilder::new()
                                            .label("Seed")
                                            .value(seed_hex_value(*state))
                                            .placeholder("#RRGGBB")
                                            .supporting_text("Press Enter to apply")
                                            .outlined()
                                            .width(Val::Percent(100.0)),
                                    );
                                });
                        });

                        spawn_copy_seed_button(row, &theme);
                    });

                // RGB sliders
                spawn_rgb_slider(panel, &theme, "R", state.r, SeedChannel::R);
                spawn_rgb_slider(panel, &theme, "G", state.g, SeedChannel::G);
                spawn_rgb_slider(panel, &theme, "B", state.b, SeedChannel::B);

                // Mode toggle
                panel.spawn((
                    Text::new("Mode"),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                ));

                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|row| {
                        spawn_mode_button(row, &theme, "Dark", ThemeMode::Dark, state.mode);
                        spawn_mode_button(row, &theme, "Light", ThemeMode::Light, state.mode);
                    });

                panel.spawn((
                    Text::new("Palette preview updates live as you adjust the seed."),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(theme.on_surface_variant),
                ));
            });

            // Preview panel
            root.spawn((
                Node {
                    flex_grow: 1.0,
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(16.0)),
                    row_gap: Val::Px(16.0),
                    overflow: Overflow::scroll(),
                    ..default()
                },
                BackgroundColor(theme.surface),
            ))
            .with_children(|panel| {
                spawn_scheme_section(panel, &theme);
                spawn_palettes_section(panel, &theme);
            });
        });
}

fn spawn_rgb_slider(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    initial: u8,
    channel: SeedChannel,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                ChannelValueText { channel },
                Text::new(format!("{label}: {initial}")),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));

            let slider = MaterialSlider::new(0.0, 255.0)
                .with_value(initial as f32)
                .with_step(1.0);

            // We can't attach extra components directly via the slider spawn helper,
            // so we tag a parent slot and attach SeedChannel to the actual slider entity
            // in a follow-up system.
            col.spawn((
                SeedSliderSlot { channel },
                Node {
                    width: Val::Percent(100.0),
                    ..default()
                },
            ))
            .with_children(|slot| {
                slot.spawn_slider_with(theme, slider, None);
            });
        });
}

fn spawn_mode_button(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    mode: ThemeMode,
    selected_mode: ThemeMode,
) {
    let selected = selected_mode == mode;
    let variant = if selected {
        ButtonVariant::FilledTonal
    } else {
        ButtonVariant::Outlined
    };

    let btn = MaterialButtonBuilder::new(label)
        .variant(variant)
        .build(theme);
    let label_color = MaterialButton::new(label)
        .with_variant(variant)
        .text_color(theme);

    parent
        .spawn((ThemeModeOption(mode), Interaction::None, btn))
        .with_children(|btn| {
            btn.spawn((
                ButtonLabel,
                Text::new(label),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(label_color),
            ));
        });
}

#[derive(Component, Clone, Copy)]
struct ThemeModeOption(ThemeMode);

fn spawn_scheme_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                Text::new("Scheme (roles)"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            for role in [
                SchemeRole::Primary,
                SchemeRole::OnPrimary,
                SchemeRole::PrimaryContainer,
                SchemeRole::OnPrimaryContainer,
                SchemeRole::Secondary,
                SchemeRole::OnSecondary,
                SchemeRole::SecondaryContainer,
                SchemeRole::OnSecondaryContainer,
                SchemeRole::Tertiary,
                SchemeRole::OnTertiary,
                SchemeRole::TertiaryContainer,
                SchemeRole::OnTertiaryContainer,
                SchemeRole::Surface,
                SchemeRole::OnSurface,
                SchemeRole::OnSurfaceVariant,
                SchemeRole::Outline,
                SchemeRole::Error,
                SchemeRole::OnError,
            ] {
                spawn_role_row(col, theme, role);
            }
        });
}

fn spawn_role_row(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme, role: SchemeRole) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Node {
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    ..default()
                },
                BorderRadius::all(Val::Px(4.0)),
                BackgroundColor(Color::NONE),
                SchemeSwatch { role },
            ));

            row.spawn((
                Text::new(role_name(role)),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            row.spawn((
                SchemeHexText { role },
                Text::new("#------"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.on_surface_variant),
            ));
        });
}

fn spawn_palettes_section(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                Text::new("Tonal palettes"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            for (label, palette) in [
                ("Primary", PaletteKind::Primary),
                ("Secondary", PaletteKind::Secondary),
                ("Tertiary", PaletteKind::Tertiary),
                ("Neutral", PaletteKind::Neutral),
                ("Neutral variant", PaletteKind::NeutralVariant),
            ] {
                spawn_palette_row(col, theme, label, palette);
            }
        });
}

fn spawn_palette_row(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    label: &str,
    palette: PaletteKind,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|col| {
            col.spawn((
                Text::new(label),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(theme.on_surface),
            ));

            col.spawn(Node {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(6.0),
                row_gap: Val::Px(6.0),
                ..default()
            })
            .with_children(|swatches| {
                for &tone in TONES {
                    swatches.spawn((
                        Node {
                            width: Val::Px(28.0),
                            height: Val::Px(28.0),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(6.0)),
                        BackgroundColor(Color::NONE),
                        PaletteSwatch { palette, tone },
                    ));
                }
            });
        });
}

fn handle_slider_changes_system(
    mut state: ResMut<PaletteToolState>,
    mut events: MessageReader<SliderChangeEvent>,
    channels: Query<&SeedChannel>,
) {
    for ev in events.read() {
        let Ok(channel) = channels.get(ev.entity) else {
            continue;
        };
        let v = ev.value.round().clamp(0.0, 255.0) as u8;
        match channel {
            SeedChannel::R => state.r = v,
            SeedChannel::G => state.g = v,
            SeedChannel::B => state.b = v,
        }
    }
}

fn attach_seed_hex_field_system(
    mut commands: Commands,
    slots: Query<(Entity, &Children), With<SeedHexFieldSlot>>,
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
        commands.entity(field_entity).insert(SeedHexField);
        commands.entity(slot_entity).remove::<SeedHexFieldSlot>();
    }
}

fn sync_controls_from_state_system(
    state: Res<PaletteToolState>,
    draft: Res<SeedHexDraftState>,
    mut sliders: Query<(&SeedChannel, &mut MaterialSlider)>,
    mut seed_field: Query<&mut MaterialTextField, With<SeedHexField>>,
) {
    if !state.is_changed() {
        return;
    }

    for (channel, mut slider) in sliders.iter_mut() {
        if slider.dragging {
            continue;
        }
        slider.value = match channel {
            SeedChannel::R => state.r as f32,
            SeedChannel::G => state.g as f32,
            SeedChannel::B => state.b as f32,
        };
    }

    let Ok(mut field) = seed_field.single_mut() else {
        return;
    };

    // If the user is actively editing, don't clobber their in-progress input.
    // If they merely focused the field (no edits yet), we still keep it in sync.
    if field.focused && draft.dirty {
        return;
    }

    field.value = seed_hex_value(*state);
    field.has_content = !field.value.is_empty();
    field.error = false;
    field.error_text = None;
}

fn handle_seed_hex_change_system(
    mut draft: ResMut<SeedHexDraftState>,
    mut change_events: MessageReader<TextFieldChangeEvent>,
    tagged: Query<(), With<SeedHexField>>,
) {
    for ev in change_events.read() {
        if tagged.get(ev.entity).is_ok() {
            draft.dirty = true;
        }
    }
}

fn handle_seed_hex_submit_system(
    mut state: ResMut<PaletteToolState>,
    mut draft: ResMut<SeedHexDraftState>,
    mut submit_events: MessageReader<TextFieldSubmitEvent>,
    tagged: Query<(), With<SeedHexField>>,
    mut fields: Query<&mut MaterialTextField>,
) {
    for ev in submit_events.read() {
        if tagged.get(ev.entity).is_err() {
            continue;
        }

        let Ok(mut field) = fields.get_mut(ev.entity) else {
            continue;
        };

        match parse_seed_hex_to_rgb(&ev.value) {
            Some((r, g, b)) => {
                state.r = r;
                state.g = g;
                state.b = b;
                field.value = seed_hex_value(*state);
                field.has_content = !field.value.is_empty();
                field.error = false;
                field.error_text = None;
                draft.dirty = false;
            }
            None => {
                field.error = true;
                field.error_text =
                    Some("Expected #RRGGBB, RRGGBB, 0xRRGGBB, or 0xFFRRGGBB".to_string());
            }
        }
    }
}

fn spawn_copy_seed_button(parent: &mut ChildSpawnerCommands, theme: &MaterialTheme) {
    let label = "Copy";
    let variant = ButtonVariant::Outlined;
    let btn = MaterialButtonBuilder::new(label)
        .variant(variant)
        .build(theme);
    let label_color = MaterialButton::new(label)
        .with_variant(variant)
        .text_color(theme);

    parent
        .spawn((CopySeedButton, Interaction::None, btn))
        .with_children(|btn| {
            btn.spawn((
                ButtonLabel,
                Text::new(label),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(label_color),
            ));
        });
}

fn handle_copy_seed_button_system(
    state: Res<PaletteToolState>,
    mut buttons: Query<&Interaction, (With<CopySeedButton>, Changed<Interaction>)>,
) {
    for interaction in buttons.iter_mut() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let seed = seed_hex_value(*state);
        if let Err(err) = copy_to_clipboard_best_effort(&seed) {
            warn!("Failed to copy seed to clipboard: {err}");
        } else {
            info!("Copied seed to clipboard: {seed}");
        }
    }
}

fn attach_seed_channels_to_sliders_system(
    mut commands: Commands,
    slots: Query<(Entity, &SeedSliderSlot, &Children)>,
    children: Query<&Children>,
    sliders: Query<(), With<MaterialSlider>>,
) {
    fn find_slider_entity(
        root: Entity,
        children_q: &Query<&Children>,
        is_slider: &Query<(), With<MaterialSlider>>,
    ) -> Option<Entity> {
        if is_slider.get(root).is_ok() {
            return Some(root);
        }
        let Ok(kids) = children_q.get(root) else {
            return None;
        };
        for child in kids.iter() {
            if let Some(found) = find_slider_entity(child, children_q, is_slider) {
                return Some(found);
            }
        }
        None
    }

    for (slot_entity, slot, slot_children) in slots.iter() {
        // Slot spawns exactly one slider hierarchy.
        let mut slider_entity: Option<Entity> = None;
        for child in slot_children.iter() {
            slider_entity = find_slider_entity(child, &children, &sliders);
            if slider_entity.is_some() {
                break;
            }
        }

        let Some(slider_entity) = slider_entity else {
            continue;
        };
        commands.entity(slider_entity).insert(slot.channel);
        // Once we attached, we can remove the slot marker to avoid re-walking the tree.
        commands.entity(slot_entity).remove::<SeedSliderSlot>();
    }
}

fn handle_mode_toggle_system(
    mut state: ResMut<PaletteToolState>,
    theme: Res<MaterialTheme>,
    mut options: Query<(&ThemeModeOption, &Interaction), Changed<Interaction>>,
) {
    for (opt, interaction) in options.iter_mut() {
        if *interaction == Interaction::Pressed {
            if state.mode != opt.0 {
                state.mode = opt.0;
            }
        }
    }

    // Keep buttons visually consistent if theme updates outside of our input.
    // (We rebuild by theme updates, so no further action is required here.)
    let _ = theme;
}

fn apply_state_to_theme_system(mut commands: Commands, state: Res<PaletteToolState>) {
    if !state.is_changed() {
        return;
    }

    commands.insert_resource(MaterialTheme::from_seed(
        Color::srgb_u8(state.r, state.g, state.b),
        state.mode,
    ));
}

fn refresh_palette_preview_system(
    state: Res<PaletteToolState>,
    theme: Res<MaterialTheme>,
    mut texts: Query<(&mut Text, Option<&SchemeHexText>, Option<&ChannelValueText>)>,
    mut swatches: Query<(
        &mut BackgroundColor,
        Option<&SeedSwatch>,
        Option<&PaletteSwatch>,
        Option<&SchemeSwatch>,
    )>,
) {
    if !state.is_changed() && !theme.is_changed() {
        return;
    }

    let seed_color = Color::srgb_u8(state.r, state.g, state.b);
    let seed_argb = argb_from_rgb_u8(state.r, state.g, state.b);

    let scheme = match state.mode {
        ThemeMode::Dark => MaterialColorScheme::dark_from_argb(seed_argb),
        ThemeMode::Light => MaterialColorScheme::light_from_argb(seed_argb),
    };

    for (mut txt, scheme_hex, channel_value) in texts.iter_mut() {
        if let Some(ch) = channel_value {
            let v = match ch.channel {
                SeedChannel::R => state.r,
                SeedChannel::G => state.g,
                SeedChannel::B => state.b,
            };
            let label = match ch.channel {
                SeedChannel::R => "R",
                SeedChannel::G => "G",
                SeedChannel::B => "B",
            };
            *txt = Text::new(format!("{label}: {v}"));
            continue;
        }

        if let Some(sw) = scheme_hex {
            let c = scheme_role_color(&scheme, sw.role);
            *txt = Text::new(color_to_hex(c));
            continue;
        }
    }

    let seed_hct = Hct::from_argb(seed_argb);
    let hue = seed_hct.hue();

    let mut primary = TonalPalette::new(hue, 48.0);
    let mut secondary = TonalPalette::new(hue, 16.0);
    let mut tertiary = TonalPalette::new((hue + 60.0) % 360.0, 24.0);
    let mut neutral = TonalPalette::new(hue, 4.0);
    let mut neutral_variant = TonalPalette::new(hue, 8.0);

    for (mut bg, is_seed, palette_sw, scheme_sw) in swatches.iter_mut() {
        if is_seed.is_some() {
            bg.0 = seed_color;
            continue;
        }

        if let Some(sw) = scheme_sw {
            bg.0 = scheme_role_color(&scheme, sw.role);
            continue;
        }

        if let Some(sw) = palette_sw {
            let argb = match sw.palette {
                PaletteKind::Primary => primary.tone(sw.tone),
                PaletteKind::Secondary => secondary.tone(sw.tone),
                PaletteKind::Tertiary => tertiary.tone(sw.tone),
                PaletteKind::Neutral => neutral.tone(sw.tone),
                PaletteKind::NeutralVariant => neutral_variant.tone(sw.tone),
            };
            bg.0 = argb_to_color(argb);
            continue;
        }
    }
}

fn seed_hex_value(state: PaletteToolState) -> String {
    format!("#{}{}{}", hex2(state.r), hex2(state.g), hex2(state.b))
}

fn hex2(v: u8) -> String {
    format!("{:02X}", v)
}

fn argb_from_rgb_u8(r: u8, g: u8, b: u8) -> u32 {
    0xFF00_0000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn argb_to_color(argb: u32) -> Color {
    let r = ((argb >> 16) & 0xFF) as f32 / 255.0;
    let g = ((argb >> 8) & 0xFF) as f32 / 255.0;
    let b = (argb & 0xFF) as f32 / 255.0;
    Color::srgb(r, g, b)
}

fn color_to_hex(color: Color) -> String {
    let srgba = color.to_srgba();
    let r = (srgba.red.clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (srgba.green.clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (srgba.blue.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn role_name(role: SchemeRole) -> &'static str {
    match role {
        SchemeRole::Primary => "primary",
        SchemeRole::OnPrimary => "on_primary",
        SchemeRole::PrimaryContainer => "primary_container",
        SchemeRole::OnPrimaryContainer => "on_primary_container",
        SchemeRole::Secondary => "secondary",
        SchemeRole::OnSecondary => "on_secondary",
        SchemeRole::SecondaryContainer => "secondary_container",
        SchemeRole::OnSecondaryContainer => "on_secondary_container",
        SchemeRole::Tertiary => "tertiary",
        SchemeRole::OnTertiary => "on_tertiary",
        SchemeRole::TertiaryContainer => "tertiary_container",
        SchemeRole::OnTertiaryContainer => "on_tertiary_container",
        SchemeRole::Surface => "surface",
        SchemeRole::OnSurface => "on_surface",
        SchemeRole::OnSurfaceVariant => "on_surface_variant",
        SchemeRole::Outline => "outline",
        SchemeRole::Error => "error",
        SchemeRole::OnError => "on_error",
    }
}

fn scheme_role_color(scheme: &MaterialColorScheme, role: SchemeRole) -> Color {
    match role {
        SchemeRole::Primary => scheme.primary,
        SchemeRole::OnPrimary => scheme.on_primary,
        SchemeRole::PrimaryContainer => scheme.primary_container,
        SchemeRole::OnPrimaryContainer => scheme.on_primary_container,
        SchemeRole::Secondary => scheme.secondary,
        SchemeRole::OnSecondary => scheme.on_secondary,
        SchemeRole::SecondaryContainer => scheme.secondary_container,
        SchemeRole::OnSecondaryContainer => scheme.on_secondary_container,
        SchemeRole::Tertiary => scheme.tertiary,
        SchemeRole::OnTertiary => scheme.on_tertiary,
        SchemeRole::TertiaryContainer => scheme.tertiary_container,
        SchemeRole::OnTertiaryContainer => scheme.on_tertiary_container,
        SchemeRole::Surface => scheme.surface,
        SchemeRole::OnSurface => scheme.on_surface,
        SchemeRole::OnSurfaceVariant => scheme.on_surface_variant,
        SchemeRole::Outline => scheme.outline,
        SchemeRole::Error => scheme.error,
        SchemeRole::OnError => scheme.on_error,
    }
}

fn parse_seed_hex_to_rgb(input: &str) -> Option<(u8, u8, u8)> {
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
            Some((r, g, b))
        }
        8 => {
            // AARRGGBB (we ignore alpha)
            let r = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let g = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let b = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some((r, g, b))
        }
        _ => None,
    }
}

fn copy_to_clipboard_best_effort(text: &str) -> Result<(), String> {
    if !cfg!(target_os = "windows") {
        return Err("Clipboard copy is only implemented on Windows in this example".to_string());
    }

    let mut child = Command::new("clip")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;

    {
        let stdin = child.stdin.as_mut().ok_or("Failed to open clip stdin")?;
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| e.to_string())?;
    }

    let status = child.wait().map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("clip exited with status {status}"))
    }
}
