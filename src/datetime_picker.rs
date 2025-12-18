//! Material Design 3 Date & Time Picker (Dialog)
//!
//! A lightweight dialog-style date+time picker built using this crate's
//! existing button/icon-button primitives.

use bevy::prelude::*;

use crate::button::{ButtonLabel, ButtonVariant, MaterialButton, MaterialButtonBuilder};
use crate::icon_button::{IconButtonBuilder, IconButtonVariant, MaterialIconButton, ICON_SIZE};
use crate::icons::{IconStyle, MaterialIcon};
use crate::scroll::{spawn_scrollbars, ScrollContainerBuilder, ScrollDirection};
use crate::theme::MaterialTheme;
use crate::tokens::{CornerRadius, Spacing};

/// Plugin for the DateTime picker component.
pub struct DateTimePickerPlugin;

impl Plugin for DateTimePickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DateTimePickerSubmitEvent>()
            .add_message::<DateTimePickerCancelEvent>()
            .add_systems(
                Update,
                (
                    datetime_picker_visibility_system,
                    datetime_picker_selector_visibility_system,
                    datetime_picker_keyboard_dismiss_system,
                    datetime_picker_month_nav_interaction_system,
                    datetime_picker_selector_toggle_interaction_system,
                    datetime_picker_year_interaction_system,
                    datetime_picker_day_interaction_system,
                    datetime_picker_time_interaction_system,
                    datetime_picker_action_interaction_system,
                    datetime_picker_dialog_render_system,
                    datetime_picker_day_grid_render_system,
                    datetime_picker_year_grid_render_system,
                    datetime_picker_theme_refresh_system,
                ),
            );
    }
}

// ============================================================================
// Date/Time types (no external dependencies)
// ============================================================================

/// Day of week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

impl Weekday {
    fn all_starting_from(first: Weekday) -> [Weekday; 7] {
        let all = [
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
        ];
        let start = all.iter().position(|d| *d == first).unwrap_or(0);
        let mut ordered = [Weekday::Sun; 7];
        for i in 0..7 {
            ordered[i] = all[(start + i) % 7];
        }
        ordered
    }

    fn short_name(self) -> &'static str {
        match self {
            Weekday::Sun => "S",
            Weekday::Mon => "M",
            Weekday::Tue => "T",
            Weekday::Wed => "W",
            Weekday::Thu => "T",
            Weekday::Fri => "F",
            Weekday::Sat => "S",
        }
    }
}

/// A calendar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn is_valid(self) -> bool {
        if !(1..=12).contains(&self.month) {
            return false;
        }
        let dim = days_in_month(self.year, self.month);
        self.day >= 1 && (self.day as u32) <= dim
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(year: i32, month: u8) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

fn month_name(month: u8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}

// Sakamoto's algorithm: returns 0=Sun..6=Sat
fn weekday_for_date(date: Date) -> Weekday {
    let mut y = date.year;
    let m = date.month as i32;
    let d = date.day as i32;
    static T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    if m < 3 {
        y -= 1;
    }
    let w = (y + y / 4 - y / 100 + y / 400 + T[(m - 1) as usize] + d) % 7;
    match w {
        0 => Weekday::Sun,
        1 => Weekday::Mon,
        2 => Weekday::Tue,
        3 => Weekday::Wed,
        4 => Weekday::Thu,
        5 => Weekday::Fri,
        _ => Weekday::Sat,
    }
}

fn weekday_index(w: Weekday) -> i32 {
    match w {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    }
}

fn add_months(mut year: i32, mut month: u8, delta: i32) -> (i32, u8) {
    let mut m = month as i32 - 1 + delta;
    year += m.div_euclid(12);
    m = m.rem_euclid(12);
    month = (m + 1) as u8;
    (year, month)
}

fn clamp_u8(value: i32, min: i32, max: i32) -> u8 {
    value.clamp(min, max) as u8
}

// ============================================================================
// Public component + builder
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFormat {
    H24,
    H12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CalendarSelector {
    Day,
    Year,
}

/// Component storing picker state.
#[derive(Component, Debug, Clone)]
pub struct MaterialDateTimePicker {
    pub open: bool,
    pub title: String,

    pub selected_date: Option<Date>,
    pub hour: u8,
    pub minute: u8,
    pub time_format: TimeFormat,

    pub display_year: i32,
    pub display_month: u8,
    pub first_day_of_week: Weekday,

    // Android MDC has a year selector mode (grid) in addition to day selection.
    selector: CalendarSelector,
    year_start: i32,
    year_end: i32,

    pub dismiss_on_scrim_click: bool,
    pub dismiss_on_escape: bool,

    pub min_date: Option<Date>,
    pub max_date: Option<Date>,
}

impl Default for MaterialDateTimePicker {
    fn default() -> Self {
        Self {
            open: false,
            title: "Select date & time".to_string(),
            selected_date: None,
            hour: 0,
            minute: 0,
            time_format: TimeFormat::H24,
            display_year: 2025,
            display_month: 1,
            first_day_of_week: Weekday::Sun,
            selector: CalendarSelector::Day,
            year_start: 1970,
            year_end: 2070,
            dismiss_on_scrim_click: true,
            dismiss_on_escape: true,
            min_date: None,
            max_date: None,
        }
    }
}

/// Builder for a DateTime picker.
#[derive(Debug, Clone)]
pub struct DateTimePickerBuilder {
    picker: MaterialDateTimePicker,
    width: Val,
}

impl Default for DateTimePickerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DateTimePickerBuilder {
    pub fn new() -> Self {
        Self {
            picker: MaterialDateTimePicker::default(),
            width: Val::Px(360.0),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.picker.title = title.into();
        self
    }

    pub fn open(mut self) -> Self {
        self.picker.open = true;
        self
    }

    pub fn date(mut self, date: Date) -> Self {
        if date.is_valid() {
            self.picker.selected_date = Some(date);
            self.picker.display_year = date.year;
            self.picker.display_month = date.month;
        }
        self
    }

    pub fn time(mut self, hour: u8, minute: u8) -> Self {
        self.picker.hour = hour % 24;
        self.picker.minute = minute % 60;
        self
    }

    pub fn time_format(mut self, fmt: TimeFormat) -> Self {
        self.picker.time_format = fmt;
        self
    }

    pub fn width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }

    pub fn first_day_of_week(mut self, first: Weekday) -> Self {
        self.picker.first_day_of_week = first;
        self
    }

    pub fn dismiss_on_scrim_click(mut self, enabled: bool) -> Self {
        self.picker.dismiss_on_scrim_click = enabled;
        self
    }

    pub fn dismiss_on_escape(mut self, enabled: bool) -> Self {
        self.picker.dismiss_on_escape = enabled;
        self
    }

    pub fn min_date(mut self, date: Date) -> Self {
        self.picker.min_date = Some(date);
        self
    }

    pub fn max_date(mut self, date: Date) -> Self {
        self.picker.max_date = Some(date);
        self
    }

    /// Override the year selector range (inclusive). If not set, it derives
    /// from min/max date when available, otherwise defaults to ±50 years around the displayed year.
    pub fn year_range(mut self, start_year: i32, end_year: i32) -> Self {
        self.picker.year_start = start_year.min(end_year);
        self.picker.year_end = start_year.max(end_year);
        self
    }

    fn build_root(
        &self,
        _theme: &MaterialTheme,
    ) -> (
        MaterialDateTimePicker,
        Node,
        Visibility,
        GlobalZIndex,
        BackgroundColor,
    ) {
        (
            self.picker.clone(),
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            if self.picker.open {
                Visibility::Visible
            } else {
                Visibility::Hidden
            },
            GlobalZIndex(9999),
            BackgroundColor(Color::NONE),
        )
    }

    fn dialog_width(&self) -> Val {
        self.width
    }

    fn finalize_year_range(mut self) -> Self {
        if let (Some(min), Some(max)) = (self.picker.min_date, self.picker.max_date) {
            self.picker.year_start = min.year.min(max.year);
            self.picker.year_end = min.year.max(max.year);
            return self;
        }

        // Reasonable default span if not constrained: +/- 50 years around the displayed year.
        let center = self.picker.display_year;
        self.picker.year_start = center - 50;
        self.picker.year_end = center + 50;
        self
    }
}

// ============================================================================
// Events
// ============================================================================

#[derive(Event, bevy::prelude::Message)]
pub struct DateTimePickerSubmitEvent {
    pub entity: Entity,
    pub date: Date,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Event, bevy::prelude::Message)]
pub struct DateTimePickerCancelEvent {
    pub entity: Entity,
}

// ============================================================================
// Internal markers
// ============================================================================

#[derive(Component)]
struct DateTimePickerScrim {
    picker: Entity,
}

#[derive(Component)]
struct DateTimePickerDialog {
    #[allow(dead_code)]
    picker: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PickerLabelKind {
    Selected,
    Month,
    Hour,
    Minute,
}

#[derive(Component)]
struct DateTimePickerLabel {
    picker: Entity,
    kind: PickerLabelKind,
}

#[derive(Component)]
struct DateTimePickerMonthNav {
    picker: Entity,
    delta: i32,
}

#[derive(Component)]
struct DateTimePickerSelectorToggle {
    picker: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimeField {
    Hour,
    Minute,
}

#[derive(Component)]
struct DateTimePickerTimeAdjust {
    picker: Entity,
    field: TimeField,
    delta: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PickerAction {
    Cancel,
    Ok,
}

#[derive(Component)]
struct DateTimePickerAction {
    picker: Entity,
    action: PickerAction,
}

#[derive(Component)]
struct DateTimePickerDayCell {
    picker: Entity,
    index: u8,
}

#[derive(Component)]
struct DateTimePickerDayCellText;

#[derive(Component)]
struct DateTimePickerDayGrid {
    picker: Entity,
}

#[derive(Component)]
struct DateTimePickerYearGrid {
    picker: Entity,
}

#[derive(Component)]
struct DateTimePickerYearCell {
    picker: Entity,
    year: i32,
}

#[derive(Component)]
struct DateTimePickerYearCellText;

// ============================================================================
// Spawn trait
// ============================================================================

pub trait SpawnDateTimePickerChild {
    fn spawn_datetime_picker(&mut self, theme: &MaterialTheme);

    fn spawn_datetime_picker_with(&mut self, theme: &MaterialTheme, builder: DateTimePickerBuilder);

    /// Spawn a date-time picker and return the spawned picker entity.
    fn spawn_datetime_picker_entity_with(
        &mut self,
        theme: &MaterialTheme,
        builder: DateTimePickerBuilder,
    ) -> Entity;
}

impl SpawnDateTimePickerChild for ChildSpawnerCommands<'_> {
    fn spawn_datetime_picker(&mut self, theme: &MaterialTheme) {
        let _ = self.spawn_datetime_picker_entity_with(theme, DateTimePickerBuilder::new());
    }

    fn spawn_datetime_picker_with(
        &mut self,
        theme: &MaterialTheme,
        builder: DateTimePickerBuilder,
    ) {
        let _ = self.spawn_datetime_picker_entity_with(theme, builder);
    }

    fn spawn_datetime_picker_entity_with(
        &mut self,
        theme: &MaterialTheme,
        builder: DateTimePickerBuilder,
    ) -> Entity {
        let builder = builder.clone().finalize_year_range();
        let first_dow = builder.picker.first_day_of_week;
        let title = builder.picker.title.clone();
        let dialog_width = builder.dialog_width();

        let mut overlay = self.spawn(builder.build_root(theme));
        let picker_entity = overlay.id();

        overlay.with_children(|overlay| {
            // Scrim
            overlay.spawn((
                DateTimePickerScrim {
                    picker: picker_entity,
                },
                Button,
                Interaction::None,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(theme.scrim.with_alpha(0.32)),
            ));

            // Dialog
            overlay
                .spawn((
                    DateTimePickerDialog {
                        picker: picker_entity,
                    },
                    Node {
                        width: dialog_width,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(Spacing::EXTRA_LARGE)),
                        row_gap: Val::Px(16.0),
                        ..default()
                    },
                    BackgroundColor(theme.surface_container_high),
                    BorderRadius::all(Val::Px(CornerRadius::EXTRA_LARGE)),
                ))
                .with_children(|dialog| {
                    dialog.spawn((
                        Text::new(title),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(theme.on_surface),
                    ));

                    dialog.spawn((
                        DateTimePickerLabel {
                            picker: picker_entity,
                            kind: PickerLabelKind::Selected,
                        },
                        Text::new("Result"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(theme.on_surface_variant),
                    ));

                    // Month nav
                    dialog
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|row| {
                            let prev_icon = "chevron_left";
                            let prev_btn = MaterialIconButton::new(prev_icon)
                                .with_variant(IconButtonVariant::Standard);
                            let prev_icon_color = prev_btn.icon_color(theme);

                            row.spawn((
                                DateTimePickerMonthNav {
                                    picker: picker_entity,
                                    delta: -1,
                                },
                                Interaction::None,
                                IconButtonBuilder::new(prev_icon)
                                    .variant(IconButtonVariant::Standard)
                                    .build(theme),
                            ))
                            .with_children(|btn| {
                                if let Some(icon) = MaterialIcon::from_name(prev_icon) {
                                    btn.spawn((
                                        icon,
                                        IconStyle::outlined()
                                            .with_color(prev_icon_color)
                                            .with_size(ICON_SIZE),
                                    ));
                                }
                            });

                            // Center label acts as toggle between DAY/YEAR selector (like MDC).
                            row.spawn((
                                DateTimePickerSelectorToggle {
                                    picker: picker_entity,
                                },
                                Button,
                                Interaction::None,
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(4.0),
                                    padding: UiRect::horizontal(Val::Px(6.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::NONE),
                            ))
                            .with_children(|toggle| {
                                toggle.spawn((
                                    DateTimePickerLabel {
                                        picker: picker_entity,
                                        kind: PickerLabelKind::Month,
                                    },
                                    Text::new(""),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface),
                                ));

                                // Dropdown indicator icon (static; we just use it to match MDC affordance).
                                if let Some(icon) = MaterialIcon::from_name("expand_more") {
                                    toggle.spawn((
                                        icon,
                                        IconStyle::outlined()
                                            .with_color(theme.on_surface_variant)
                                            .with_size(18.0),
                                    ));
                                }
                            });

                            let next_icon = "chevron_right";
                            let next_btn = MaterialIconButton::new(next_icon)
                                .with_variant(IconButtonVariant::Standard);
                            let next_icon_color = next_btn.icon_color(theme);

                            row.spawn((
                                DateTimePickerMonthNav {
                                    picker: picker_entity,
                                    delta: 1,
                                },
                                Interaction::None,
                                IconButtonBuilder::new(next_icon)
                                    .variant(IconButtonVariant::Standard)
                                    .build(theme),
                            ))
                            .with_children(|btn| {
                                if let Some(icon) = MaterialIcon::from_name(next_icon) {
                                    btn.spawn((
                                        icon,
                                        IconStyle::outlined()
                                            .with_color(next_icon_color)
                                            .with_size(ICON_SIZE),
                                    ));
                                }
                            });
                        });

                    // Weekday header
                    dialog
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        })
                        .with_children(|row| {
                            for wd in Weekday::all_starting_from(first_dow) {
                                row.spawn((
                                    Text::new(wd.short_name()),
                                    TextFont {
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface_variant),
                                    Node {
                                        width: Val::Px(40.0),
                                        ..default()
                                    },
                                ));
                            }
                        });

                    // Day grid
                    dialog
                        .spawn((
                            DateTimePickerDayGrid {
                                picker: picker_entity,
                            },
                            Node {
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::Wrap,
                                column_gap: Val::Px(0.0),
                                row_gap: Val::Px(0.0),
                                ..default()
                            },
                        ))
                        .with_children(|grid| {
                            for index in 0..42u8 {
                                grid.spawn((
                                    DateTimePickerDayCell {
                                        picker: picker_entity,
                                        index,
                                    },
                                    Button,
                                    Interaction::None,
                                    Node {
                                        width: Val::Px(40.0),
                                        height: Val::Px(40.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(Color::NONE),
                                    BorderRadius::all(Val::Px(CornerRadius::FULL)),
                                ))
                                .with_children(|cell| {
                                    cell.spawn((
                                        DateTimePickerDayCellText,
                                        Text::new(""),
                                        TextFont {
                                            font_size: 14.0,
                                            ..default()
                                        },
                                        TextColor(theme.on_surface),
                                    ));
                                });
                            }
                        });

                    // Year selector (Android-style): scrollable year grid.
                    // Starts hidden and is toggled by clicking the month/year header.
                    dialog
                        .spawn((
                            DateTimePickerYearGrid {
                                picker: picker_entity,
                            },
                            ScrollContainerBuilder::new()
                                .vertical()
                                .with_scrollbars(true)
                                .build(),
                            ScrollPosition::default(),
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(240.0),
                                overflow: Overflow::scroll_y(),
                                ..default()
                            },
                            Visibility::Hidden,
                        ))
                        .with_children(|years_container| {
                            // Visual scrollbars match existing crate behavior.
                            spawn_scrollbars(years_container, theme, ScrollDirection::Vertical);

                            // A wrap grid of year buttons.
                            years_container
                                .spawn(Node {
                                    width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    justify_content: JustifyContent::FlexStart,
                                    column_gap: Val::Px(8.0),
                                    row_gap: Val::Px(8.0),
                                    padding: UiRect::all(Val::Px(8.0)),
                                    ..default()
                                })
                                .with_children(|grid| {
                                    let start = builder.picker.year_start;
                                    let end = builder.picker.year_end;
                                    for year in start..=end {
                                        grid.spawn((
                                            DateTimePickerYearCell {
                                                picker: picker_entity,
                                                year,
                                            },
                                            Button,
                                            Interaction::None,
                                            Node {
                                                width: Val::Px(72.0),
                                                height: Val::Px(40.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            BackgroundColor(Color::NONE),
                                            BorderRadius::all(Val::Px(20.0)),
                                        ))
                                        .with_children(
                                            |cell| {
                                                cell.spawn((
                                                    DateTimePickerYearCellText,
                                                    Text::new(""),
                                                    TextFont {
                                                        font_size: 14.0,
                                                        ..default()
                                                    },
                                                    TextColor(theme.on_surface),
                                                ));
                                            },
                                        );
                                    }
                                });
                        });

                    // Time controls
                    dialog
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|col| {
                            col.spawn((
                                Text::new("Time"),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(theme.on_surface_variant),
                            ));

                            col.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(Spacing::SMALL),
                                align_items: AlignItems::Center,
                                ..default()
                            })
                            .with_children(|row| {
                                spawn_time_adjust(row, theme, picker_entity, TimeField::Hour, -1);
                                row.spawn((
                                    DateTimePickerLabel {
                                        picker: picker_entity,
                                        kind: PickerLabelKind::Hour,
                                    },
                                    Text::new("00"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface),
                                ));
                                spawn_time_adjust(row, theme, picker_entity, TimeField::Hour, 1);

                                row.spawn((
                                    Text::new(":"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface),
                                ));

                                spawn_time_adjust(row, theme, picker_entity, TimeField::Minute, -1);
                                row.spawn((
                                    DateTimePickerLabel {
                                        picker: picker_entity,
                                        kind: PickerLabelKind::Minute,
                                    },
                                    Text::new("00"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(theme.on_surface),
                                ));
                                spawn_time_adjust(row, theme, picker_entity, TimeField::Minute, 1);
                            });
                        });

                    // Actions
                    dialog
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::End,
                            column_gap: Val::Px(Spacing::SMALL),
                            ..default()
                        })
                        .with_children(|row| {
                            spawn_action_button(
                                row,
                                theme,
                                picker_entity,
                                PickerAction::Cancel,
                                "Cancel",
                            );
                            spawn_action_button(row, theme, picker_entity, PickerAction::Ok, "OK");
                        });
                });
        });

        picker_entity
    }
}

fn spawn_time_adjust(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    picker: Entity,
    field: TimeField,
    delta: i32,
) {
    let icon = if delta < 0 { "remove" } else { "add" };
    let btn = MaterialIconButton::new(icon).with_variant(IconButtonVariant::Standard);
    let icon_color = btn.icon_color(theme);

    parent
        .spawn((
            DateTimePickerTimeAdjust {
                picker,
                field,
                delta,
            },
            Interaction::None,
            IconButtonBuilder::new(icon)
                .variant(IconButtonVariant::Standard)
                .build(theme),
        ))
        .with_children(|btn| {
            if let Some(icon) = MaterialIcon::from_name(icon) {
                btn.spawn((
                    icon,
                    IconStyle::outlined()
                        .with_color(icon_color)
                        .with_size(ICON_SIZE),
                ));
            }
        });
}

fn spawn_action_button(
    parent: &mut ChildSpawnerCommands,
    theme: &MaterialTheme,
    picker: Entity,
    action: PickerAction,
    label: &'static str,
) {
    let btn = MaterialButton::new(label).with_variant(ButtonVariant::Text);
    let text_color = btn.text_color(theme);

    parent
        .spawn((
            DateTimePickerAction { picker, action },
            Interaction::None,
            MaterialButtonBuilder::new(label).text().build(theme),
        ))
        .with_children(|btn| {
            btn.spawn((
                ButtonLabel,
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(text_color),
            ));
        });
}

// ============================================================================
// Systems
// ============================================================================

fn datetime_picker_visibility_system(
    mut pickers: Query<(&MaterialDateTimePicker, &mut Visibility), Changed<MaterialDateTimePicker>>,
) {
    for (picker, mut vis) in pickers.iter_mut() {
        *vis = if picker.open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn datetime_picker_selector_visibility_system(
    pickers: Query<&MaterialDateTimePicker>,
    mut grids: Query<(
        &mut Visibility,
        Option<&DateTimePickerDayGrid>,
        Option<&DateTimePickerYearGrid>,
    )>,
) {
    for (mut vis, day, year) in grids.iter_mut() {
        let picker_entity = if let Some(day) = day {
            day.picker
        } else if let Some(year) = year {
            year.picker
        } else {
            continue;
        };

        let Ok(picker) = pickers.get(picker_entity) else {
            continue;
        };

        if day.is_some() {
            *vis = if picker.open && picker.selector == CalendarSelector::Day {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        } else if year.is_some() {
            *vis = if picker.open && picker.selector == CalendarSelector::Year {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn datetime_picker_keyboard_dismiss_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut pickers: Query<(Entity, &mut MaterialDateTimePicker)>,
    mut cancel: MessageWriter<DateTimePickerCancelEvent>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    for (entity, mut picker) in pickers.iter_mut() {
        if picker.open && picker.dismiss_on_escape {
            picker.open = false;
            cancel.write(DateTimePickerCancelEvent { entity });
        }
    }
}

fn datetime_picker_month_nav_interaction_system(
    mut pickers: Query<&mut MaterialDateTimePicker>,
    nav: Query<(&Interaction, &DateTimePickerMonthNav), Changed<Interaction>>,
) {
    for (interaction, nav) in nav.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Ok(mut picker) = pickers.get_mut(nav.picker) else {
            continue;
        };
        if !picker.open {
            continue;
        }
        // Match MDC: month paging is part of day selection mode.
        if picker.selector != CalendarSelector::Day {
            continue;
        }
        let (y, m) = add_months(picker.display_year, picker.display_month, nav.delta);
        picker.display_year = y;
        picker.display_month = m;
    }
}

fn datetime_picker_selector_toggle_interaction_system(
    mut pickers: Query<&mut MaterialDateTimePicker>,
    toggles: Query<(&Interaction, &DateTimePickerSelectorToggle), Changed<Interaction>>,
) {
    for (interaction, toggle) in toggles.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Ok(mut picker) = pickers.get_mut(toggle.picker) else {
            continue;
        };
        if !picker.open {
            continue;
        }
        picker.selector = match picker.selector {
            CalendarSelector::Day => CalendarSelector::Year,
            CalendarSelector::Year => CalendarSelector::Day,
        };
    }
}

fn datetime_picker_year_interaction_system(
    mut pickers: Query<&mut MaterialDateTimePicker>,
    years: Query<(&Interaction, &DateTimePickerYearCell), Changed<Interaction>>,
) {
    for (interaction, cell) in years.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Ok(mut picker) = pickers.get_mut(cell.picker) else {
            continue;
        };
        if !picker.open {
            continue;
        }
        if picker.selector != CalendarSelector::Year {
            continue;
        }
        let year = cell.year.clamp(picker.year_start, picker.year_end);
        picker.display_year = year;
        // Match MDC behavior: selecting a year returns to day selection.
        picker.selector = CalendarSelector::Day;
    }
}

fn datetime_picker_day_interaction_system(
    mut pickers: Query<&mut MaterialDateTimePicker>,
    days: Query<(&Interaction, &DateTimePickerDayCell), Changed<Interaction>>,
) {
    for (interaction, cell) in days.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Ok(mut picker) = pickers.get_mut(cell.picker) else {
            continue;
        };
        if !picker.open {
            continue;
        }
        if picker.selector != CalendarSelector::Day {
            continue;
        }

        let first = Date::new(picker.display_year, picker.display_month, 1);
        let first_wd = weekday_for_date(first);
        let offset =
            (weekday_index(first_wd) - weekday_index(picker.first_day_of_week)).rem_euclid(7);
        let day_number = cell.index as i32 - offset + 1;

        let dim = days_in_month(picker.display_year, picker.display_month) as i32;
        if !(1..=dim).contains(&day_number) {
            continue;
        }

        let selected = Date::new(picker.display_year, picker.display_month, day_number as u8);
        if !selected.is_valid() {
            continue;
        }

        // Optional min/max constraints
        if let Some(min) = picker.min_date {
            if selected < min {
                continue;
            }
        }
        if let Some(max) = picker.max_date {
            if selected > max {
                continue;
            }
        }

        picker.selected_date = Some(selected);
    }
}

fn datetime_picker_time_interaction_system(
    mut pickers: Query<&mut MaterialDateTimePicker>,
    adjust: Query<(&Interaction, &DateTimePickerTimeAdjust), Changed<Interaction>>,
) {
    for (interaction, adjust) in adjust.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Ok(mut picker) = pickers.get_mut(adjust.picker) else {
            continue;
        };
        if !picker.open {
            continue;
        }

        match adjust.field {
            TimeField::Hour => {
                let new_hour = (picker.hour as i32 + adjust.delta).rem_euclid(24);
                picker.hour = new_hour as u8;
            }
            TimeField::Minute => {
                let new_min = (picker.minute as i32 + adjust.delta).rem_euclid(60);
                picker.minute = new_min as u8;
            }
        }
    }
}

fn datetime_picker_action_interaction_system(
    mut pickers: Query<&mut MaterialDateTimePicker>,
    actions: Query<(&Interaction, &DateTimePickerAction), Changed<Interaction>>,
    scrim: Query<(&Interaction, &DateTimePickerScrim), Changed<Interaction>>,
    mut submit: MessageWriter<DateTimePickerSubmitEvent>,
    mut cancel: MessageWriter<DateTimePickerCancelEvent>,
) {
    for (interaction, scrim) in scrim.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let Ok(mut picker) = pickers.get_mut(scrim.picker) else {
            continue;
        };
        if picker.open && picker.dismiss_on_scrim_click {
            picker.open = false;
            cancel.write(DateTimePickerCancelEvent {
                entity: scrim.picker,
            });
        }
    }

    for (interaction, action) in actions.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Ok(mut picker) = pickers.get_mut(action.picker) else {
            continue;
        };
        if !picker.open {
            continue;
        }

        match action.action {
            PickerAction::Cancel => {
                picker.open = false;
                cancel.write(DateTimePickerCancelEvent {
                    entity: action.picker,
                });
            }
            PickerAction::Ok => {
                if let Some(date) = picker.selected_date {
                    picker.open = false;
                    submit.write(DateTimePickerSubmitEvent {
                        entity: action.picker,
                        date,
                        hour: picker.hour,
                        minute: picker.minute,
                    });
                }
            }
        }
    }
}

fn datetime_picker_dialog_render_system(
    theme: Res<MaterialTheme>,
    pickers: Query<&MaterialDateTimePicker>,
    mut labels: Query<(&DateTimePickerLabel, &mut Text, &mut TextColor)>,
) {
    let theme_changed = theme.is_changed();

    for (label, mut text, mut color) in labels.iter_mut() {
        let Ok(picker) = pickers.get(label.picker) else {
            continue;
        };
        if !picker.open && !theme_changed {
            continue;
        }

        match label.kind {
            PickerLabelKind::Month => {
                // In year selector mode, match MDC by emphasizing year.
                if picker.selector == CalendarSelector::Year {
                    *text = Text::new(format!("{}", picker.display_year));
                } else {
                    *text = Text::new(format!(
                        "{} {}",
                        month_name(picker.display_month),
                        picker.display_year
                    ));
                }
                *color = TextColor(theme.on_surface);
            }
            PickerLabelKind::Selected => {
                let date_part = if let Some(d) = picker.selected_date {
                    format!("{:04}-{:02}-{:02}", d.year, d.month, d.day)
                } else {
                    "No date".to_string()
                };
                let (hour_display, suffix) = match picker.time_format {
                    TimeFormat::H24 => (picker.hour as i32, ""),
                    TimeFormat::H12 => {
                        let mut h = picker.hour as i32;
                        let am = h < 12;
                        if h == 0 {
                            h = 12;
                        } else if h > 12 {
                            h -= 12;
                        }
                        (h, if am { " AM" } else { " PM" })
                    }
                };

                *text = Text::new(format!(
                    "{}  •  {:02}:{:02}{}",
                    date_part, hour_display, picker.minute, suffix
                ));
                *color = TextColor(theme.on_surface_variant);
            }
            PickerLabelKind::Hour => {
                let (hour_display, _) = match picker.time_format {
                    TimeFormat::H24 => (picker.hour as i32, ""),
                    TimeFormat::H12 => {
                        let mut h = picker.hour as i32;
                        if h == 0 {
                            h = 12;
                        } else if h > 12 {
                            h -= 12;
                        }
                        (h, "")
                    }
                };
                *text = Text::new(format!("{:02}", clamp_u8(hour_display, 0, 23)));
                *color = TextColor(theme.on_surface);
            }
            PickerLabelKind::Minute => {
                *text = Text::new(format!("{:02}", picker.minute));
                *color = TextColor(theme.on_surface);
            }
        }
    }
}

fn datetime_picker_year_grid_render_system(
    theme: Res<MaterialTheme>,
    pickers: Query<&MaterialDateTimePicker>,
    mut years: Query<(&DateTimePickerYearCell, &mut BackgroundColor, &Children)>,
    mut texts: Query<(&DateTimePickerYearCellText, &mut Text, &mut TextColor)>,
) {
    let theme_changed = theme.is_changed();

    for (cell, mut bg, children) in years.iter_mut() {
        let Ok(picker) = pickers.get(cell.picker) else {
            continue;
        };
        if !picker.open && !theme_changed {
            continue;
        }

        let selected_year = picker.selected_date.map(|d| d.year);
        let is_selected = selected_year == Some(cell.year);

        *bg = if is_selected {
            BackgroundColor(theme.primary)
        } else {
            BackgroundColor(Color::NONE)
        };

        for child in children.iter() {
            if let Ok((_marker, mut text, mut color)) = texts.get_mut(child) {
                *text = Text::new(format!("{}", cell.year));
                *color = if is_selected {
                    TextColor(theme.on_primary)
                } else {
                    TextColor(theme.on_surface)
                };
            }
        }
    }
}

fn datetime_picker_day_grid_render_system(
    theme: Res<MaterialTheme>,
    pickers: Query<&MaterialDateTimePicker>,
    mut cells: Query<(&DateTimePickerDayCell, &mut BackgroundColor, &Children)>,
    mut texts: Query<(&DateTimePickerDayCellText, &mut Text, &mut TextColor)>,
) {
    let theme_changed = theme.is_changed();

    for (cell, mut bg, children) in cells.iter_mut() {
        let Ok(picker) = pickers.get(cell.picker) else {
            continue;
        };
        if !picker.open && !theme_changed {
            continue;
        }

        let first = Date::new(picker.display_year, picker.display_month, 1);
        let first_wd = weekday_for_date(first);
        let offset =
            (weekday_index(first_wd) - weekday_index(picker.first_day_of_week)).rem_euclid(7);
        let day_number = cell.index as i32 - offset + 1;

        let dim = days_in_month(picker.display_year, picker.display_month) as i32;
        let valid = (1..=dim).contains(&day_number);

        let cell_date = if valid {
            Some(Date::new(
                picker.display_year,
                picker.display_month,
                day_number as u8,
            ))
        } else {
            None
        };

        let is_selected = cell_date.is_some() && picker.selected_date == cell_date;
        *bg = if is_selected {
            BackgroundColor(theme.primary)
        } else {
            BackgroundColor(Color::NONE)
        };

        for child in children.iter() {
            if let Ok((_marker, mut text, mut color)) = texts.get_mut(child) {
                if let Some(d) = cell_date {
                    *text = Text::new(format!("{}", d.day));
                    *color = if is_selected {
                        TextColor(theme.on_primary)
                    } else {
                        TextColor(theme.on_surface)
                    };
                } else {
                    *text = Text::new("");
                    *color = TextColor(theme.on_surface_variant);
                }
            }
        }
    }
}

fn datetime_picker_theme_refresh_system(
    theme: Res<MaterialTheme>,
    mut backgrounds: Query<(
        &mut BackgroundColor,
        Option<&DateTimePickerScrim>,
        Option<&DateTimePickerDialog>,
    )>,
) {
    if !theme.is_changed() {
        return;
    }

    for (mut bg, scrim, dialog) in backgrounds.iter_mut() {
        if scrim.is_some() {
            *bg = BackgroundColor(theme.scrim.with_alpha(0.32));
        } else if dialog.is_some() {
            *bg = BackgroundColor(theme.surface_container_high);
        }
    }
}
