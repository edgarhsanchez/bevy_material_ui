//! Material Icon Component
//!
//! Provides the main icon component and bundle for rendering Material Design icons as embedded bitmaps.

use super::codepoints::*;
use super::style::IconStyle;
use super::MaterialIconFont;
use super::EMBEDDED_MATERIAL_SYMBOLS_FONT;
use bevy::prelude::*;

/// A Material Design icon component
///
/// This component represents a Material Design icon by its identifier.
/// Icons are rendered as embedded bitmap images from the `google-material-design-icons-bin` crate.
///
/// # Example
///
/// ```rust,ignore
/// use bevy::prelude::*;
/// use bevy_material_ui::icons::{MaterialIcon, IconBundle};
///
/// fn spawn_icon(mut commands: Commands) {
///     commands.spawn(IconBundle {
///         icon: MaterialIcon::home(),
///         ..default()
///     });
/// }
/// ```
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialIcon {
    /// Unicode codepoint of the icon
    pub codepoint: char,
}

impl Default for MaterialIcon {
    fn default() -> Self {
        Self::home()
    }
}

impl MaterialIcon {
    /// Create a new icon from a codepoint
    pub fn new(codepoint: char) -> Self {
        Self { codepoint }
    }

    /// Create an icon from a name
    ///
    /// Returns None if the name is not recognized.
    pub fn from_name(name: &str) -> Option<Self> {
        icon_by_name(name).map(Self::new)
    }

    /// Get the icon as a string (single character)
    pub fn as_str(&self) -> String {
        self.codepoint.to_string()
    }

    // Navigation icons
    /// Home icon
    pub fn home() -> Self {
        Self::new(ICON_HOME)
    }
    /// Menu icon (hamburger)
    pub fn menu() -> Self {
        Self::new(ICON_MENU)
    }
    /// More vertical icon (three dots)
    pub fn more_vert() -> Self {
        Self::new(ICON_MORE_VERT)
    }
    /// More horizontal icon
    pub fn more_horiz() -> Self {
        Self::new(ICON_MORE_HORIZ)
    }
    /// Arrow back icon
    pub fn arrow_back() -> Self {
        Self::new(ICON_ARROW_BACK)
    }
    /// Arrow forward icon
    pub fn arrow_forward() -> Self {
        Self::new(ICON_ARROW_FORWARD)
    }
    /// Arrow upward icon
    pub fn arrow_upward() -> Self {
        Self::new(ICON_ARROW_UPWARD)
    }
    /// Arrow downward icon
    pub fn arrow_downward() -> Self {
        Self::new(ICON_ARROW_DOWNWARD)
    }
    /// Close icon
    pub fn close() -> Self {
        Self::new(ICON_CLOSE)
    }
    /// Check icon
    pub fn check() -> Self {
        Self::new(ICON_CHECK)
    }
    /// Expand more icon (chevron down)
    pub fn expand_more() -> Self {
        Self::new(ICON_EXPAND_MORE)
    }
    /// Expand less icon (chevron up)
    pub fn expand_less() -> Self {
        Self::new(ICON_EXPAND_LESS)
    }
    /// Chevron left icon
    pub fn chevron_left() -> Self {
        Self::new(ICON_CHEVRON_LEFT)
    }
    /// Chevron right icon
    pub fn chevron_right() -> Self {
        Self::new(ICON_CHEVRON_RIGHT)
    }

    // Action icons
    /// Add icon (plus)
    pub fn add() -> Self {
        Self::new(ICON_ADD)
    }
    /// Remove icon (minus)
    pub fn remove() -> Self {
        Self::new(ICON_REMOVE)
    }
    /// Delete icon (trash)
    pub fn delete() -> Self {
        Self::new(ICON_DELETE)
    }
    /// Edit icon (pencil)
    pub fn edit() -> Self {
        Self::new(ICON_EDIT)
    }
    /// Save icon
    pub fn save() -> Self {
        Self::new(ICON_SAVE)
    }
    /// Search icon
    pub fn search() -> Self {
        Self::new(ICON_SEARCH)
    }
    /// Refresh icon
    pub fn refresh() -> Self {
        Self::new(ICON_REFRESH)
    }
    /// Settings icon (gear)
    pub fn settings() -> Self {
        Self::new(ICON_SETTINGS)
    }
    /// Help icon (question mark)
    pub fn help() -> Self {
        Self::new(ICON_HELP)
    }
    /// Info icon
    pub fn info() -> Self {
        Self::new(ICON_INFO)
    }
    /// Share icon
    pub fn share() -> Self {
        Self::new(ICON_SHARE)
    }
    /// Download icon
    pub fn download() -> Self {
        Self::new(ICON_DOWNLOAD)
    }
    /// Upload icon
    pub fn upload() -> Self {
        Self::new(ICON_UPLOAD)
    }
    /// Print icon
    pub fn print() -> Self {
        Self::new(ICON_PRINT)
    }
    /// Copy icon
    pub fn copy() -> Self {
        Self::new(ICON_CONTENT_COPY)
    }
    /// Paste icon
    pub fn paste() -> Self {
        Self::new(ICON_CONTENT_PASTE)
    }
    /// Cut icon
    pub fn cut() -> Self {
        Self::new(ICON_CONTENT_CUT)
    }
    /// Undo icon
    pub fn undo() -> Self {
        Self::new(ICON_UNDO)
    }
    /// Redo icon
    pub fn redo() -> Self {
        Self::new(ICON_REDO)
    }

    // Toggle icons
    /// Checkbox checked icon
    pub fn checkbox_checked() -> Self {
        Self::new(ICON_CHECK_BOX)
    }
    /// Checkbox unchecked icon
    pub fn checkbox_unchecked() -> Self {
        Self::new(ICON_CHECK_BOX_OUTLINE_BLANK)
    }
    /// Radio button checked icon
    pub fn radio_checked() -> Self {
        Self::new(ICON_RADIO_BUTTON_CHECKED)
    }
    /// Radio button unchecked icon
    pub fn radio_unchecked() -> Self {
        Self::new(ICON_RADIO_BUTTON_UNCHECKED)
    }
    /// Toggle on icon
    pub fn toggle_on() -> Self {
        Self::new(ICON_TOGGLE_ON)
    }
    /// Toggle off icon
    pub fn toggle_off() -> Self {
        Self::new(ICON_TOGGLE_OFF)
    }
    /// Star filled icon
    pub fn star() -> Self {
        Self::new(ICON_STAR)
    }
    /// Star outline icon
    pub fn star_outline() -> Self {
        Self::new(ICON_STAR_BORDER)
    }
    /// Favorite (heart) filled icon
    pub fn favorite() -> Self {
        Self::new(ICON_FAVORITE)
    }
    /// Favorite outline icon
    pub fn favorite_outline() -> Self {
        Self::new(ICON_FAVORITE_BORDER)
    }
    /// Visibility (eye) icon
    pub fn visibility() -> Self {
        Self::new(ICON_VISIBILITY)
    }
    /// Visibility off icon
    pub fn visibility_off() -> Self {
        Self::new(ICON_VISIBILITY_OFF)
    }

    // Alert/Status icons
    /// Error icon
    pub fn error() -> Self {
        Self::new(ICON_ERROR)
    }
    /// Warning icon
    pub fn warning() -> Self {
        Self::new(ICON_WARNING)
    }
    /// Check circle (success) icon
    pub fn check_circle() -> Self {
        Self::new(ICON_CHECK_CIRCLE)
    }
    /// Cancel icon
    pub fn cancel() -> Self {
        Self::new(ICON_CANCEL)
    }
    /// Block icon
    pub fn block() -> Self {
        Self::new(ICON_BLOCK)
    }
    /// Notifications (bell) icon
    pub fn notifications() -> Self {
        Self::new(ICON_NOTIFICATIONS)
    }
    /// Notifications off icon
    pub fn notifications_off() -> Self {
        Self::new(ICON_NOTIFICATIONS_OFF)
    }

    // Content icons
    /// Folder icon
    pub fn folder() -> Self {
        Self::new(ICON_FOLDER)
    }
    /// Folder open icon
    pub fn folder_open() -> Self {
        Self::new(ICON_FOLDER_OPEN)
    }
    /// Document icon
    pub fn document() -> Self {
        Self::new(ICON_DESCRIPTION)
    }
    /// Image icon
    pub fn image() -> Self {
        Self::new(ICON_IMAGE)
    }
    /// Video icon
    pub fn video() -> Self {
        Self::new(ICON_VIDEOCAM)
    }
    /// Music note icon
    pub fn music() -> Self {
        Self::new(ICON_MUSIC_NOTE)
    }
    /// Link icon
    pub fn link() -> Self {
        Self::new(ICON_LINK)
    }
    /// Attachment icon
    pub fn attachment() -> Self {
        Self::new(ICON_ATTACH_FILE)
    }

    // Person/Account icons
    /// Person icon
    pub fn person() -> Self {
        Self::new(ICON_PERSON)
    }
    /// Group (people) icon
    pub fn group() -> Self {
        Self::new(ICON_GROUP)
    }
    /// Account circle icon
    pub fn account_circle() -> Self {
        Self::new(ICON_ACCOUNT_CIRCLE)
    }
    /// Person add icon
    pub fn person_add() -> Self {
        Self::new(ICON_PERSON_ADD)
    }
    /// Login icon
    pub fn login() -> Self {
        Self::new(ICON_LOGIN)
    }
    /// Logout icon
    pub fn logout() -> Self {
        Self::new(ICON_LOGOUT)
    }

    // Communication icons
    /// Email icon
    pub fn email() -> Self {
        Self::new(ICON_EMAIL)
    }
    /// Chat icon
    pub fn chat() -> Self {
        Self::new(ICON_CHAT)
    }
    /// Message icon
    pub fn message() -> Self {
        Self::new(ICON_MESSAGE)
    }
    /// Phone icon
    pub fn phone() -> Self {
        Self::new(ICON_PHONE)
    }
    /// Send icon
    pub fn send() -> Self {
        Self::new(ICON_SEND)
    }

    // Media control icons
    /// Play icon
    pub fn play() -> Self {
        Self::new(ICON_PLAY_ARROW)
    }
    /// Pause icon
    pub fn pause() -> Self {
        Self::new(ICON_PAUSE)
    }
    /// Stop icon
    pub fn stop() -> Self {
        Self::new(ICON_STOP)
    }
    /// Skip next icon
    pub fn skip_next() -> Self {
        Self::new(ICON_SKIP_NEXT)
    }
    /// Skip previous icon
    pub fn skip_previous() -> Self {
        Self::new(ICON_SKIP_PREVIOUS)
    }
    /// Fast forward icon
    pub fn fast_forward() -> Self {
        Self::new(ICON_FAST_FORWARD)
    }
    /// Fast rewind icon
    pub fn fast_rewind() -> Self {
        Self::new(ICON_FAST_REWIND)
    }
    /// Replay icon
    pub fn replay() -> Self {
        Self::new(ICON_REPLAY)
    }
    /// Shuffle icon
    pub fn shuffle() -> Self {
        Self::new(ICON_SHUFFLE)
    }
    /// Repeat icon
    pub fn repeat_icon() -> Self {
        Self::new(ICON_REPEAT)
    }
    /// Volume up icon
    pub fn volume_up() -> Self {
        Self::new(ICON_VOLUME_UP)
    }
    /// Volume down icon
    pub fn volume_down() -> Self {
        Self::new(ICON_VOLUME_DOWN)
    }
    /// Volume mute icon
    pub fn volume_mute() -> Self {
        Self::new(ICON_VOLUME_MUTE)
    }
    /// Volume off icon
    pub fn volume_off() -> Self {
        Self::new(ICON_VOLUME_OFF)
    }

    // Device icons
    /// Smartphone icon
    pub fn smartphone() -> Self {
        Self::new(ICON_SMARTPHONE)
    }
    /// Tablet icon
    pub fn tablet() -> Self {
        Self::new(ICON_TABLET)
    }
    /// Laptop icon
    pub fn laptop() -> Self {
        Self::new(ICON_LAPTOP)
    }
    /// Desktop icon
    pub fn desktop() -> Self {
        Self::new(ICON_DESKTOP_WINDOWS)
    }
    /// Keyboard icon
    pub fn keyboard() -> Self {
        Self::new(ICON_KEYBOARD)
    }
    /// Mouse icon
    pub fn mouse() -> Self {
        Self::new(ICON_MOUSE)
    }
    /// Gamepad icon
    pub fn gamepad() -> Self {
        Self::new(ICON_GAMEPAD)
    }
    /// Wifi icon
    pub fn wifi() -> Self {
        Self::new(ICON_WIFI)
    }
    /// Bluetooth icon
    pub fn bluetooth() -> Self {
        Self::new(ICON_BLUETOOTH)
    }
    /// Battery full icon
    pub fn battery_full() -> Self {
        Self::new(ICON_BATTERY_FULL)
    }
    /// Battery alert icon
    pub fn battery_alert() -> Self {
        Self::new(ICON_BATTERY_ALERT)
    }

    // Game/D&D icons
    /// Dice icon
    pub fn dice() -> Self {
        Self::new(ICON_CASINO)
    }
    /// Puzzle/module icon
    pub fn puzzle() -> Self {
        Self::new(ICON_EXTENSION)
    }
    /// Shield icon
    pub fn shield() -> Self {
        Self::new(ICON_SHIELD)
    }
    /// Combat/martial arts icon
    pub fn combat() -> Self {
        Self::new(ICON_SPORTS_MARTIAL_ARTS)
    }
    /// Magic/spell icon
    pub fn magic() -> Self {
        Self::new(ICON_AUTO_FIX_HIGH)
    }
    /// Lightbulb/inspiration icon
    pub fn lightbulb() -> Self {
        Self::new(ICON_LIGHTBULB)
    }
    /// Inventory/backpack icon
    pub fn inventory() -> Self {
        Self::new(ICON_INVENTORY_2)
    }
    /// Book/spellbook icon
    pub fn book() -> Self {
        Self::new(ICON_BOOK)
    }
    /// Mind/wisdom icon
    pub fn mind() -> Self {
        Self::new(ICON_PSYCHOLOGY)
    }
    /// Strength icon
    pub fn strength() -> Self {
        Self::new(ICON_FITNESS_CENTER)
    }
    /// Speed/dexterity icon
    pub fn speed() -> Self {
        Self::new(ICON_SPEED)
    }
    /// Health/healing icon
    pub fn health() -> Self {
        Self::new(ICON_HEALING)
    }

    // Misc icons
    /// Language/globe icon
    pub fn language() -> Self {
        Self::new(ICON_LANGUAGE)
    }
    /// Dark mode icon
    pub fn dark_mode() -> Self {
        Self::new(ICON_DARK_MODE)
    }
    /// Light mode icon
    pub fn light_mode() -> Self {
        Self::new(ICON_LIGHT_MODE)
    }
    /// Fullscreen icon
    pub fn fullscreen() -> Self {
        Self::new(ICON_FULLSCREEN)
    }
    /// Fullscreen exit icon
    pub fn fullscreen_exit() -> Self {
        Self::new(ICON_FULLSCREEN_EXIT)
    }
    /// Zoom in icon
    pub fn zoom_in() -> Self {
        Self::new(ICON_ZOOM_IN)
    }
    /// Zoom out icon
    pub fn zoom_out() -> Self {
        Self::new(ICON_ZOOM_OUT)
    }
    /// Lock icon
    pub fn lock() -> Self {
        Self::new(ICON_LOCK)
    }
    /// Lock open icon
    pub fn lock_open() -> Self {
        Self::new(ICON_LOCK_OPEN)
    }
    /// Tune/adjust icon
    pub fn tune() -> Self {
        Self::new(ICON_TUNE)
    }
    /// Filter icon
    pub fn filter() -> Self {
        Self::new(ICON_FILTER_LIST)
    }
    /// Sort icon
    pub fn sort() -> Self {
        Self::new(ICON_SORT)
    }
    /// Drag handle icon
    pub fn drag_handle() -> Self {
        Self::new(ICON_DRAG_HANDLE)
    }
    /// Apps/grid icon
    pub fn apps() -> Self {
        Self::new(ICON_APPS)
    }
    /// List view icon
    pub fn list_view() -> Self {
        Self::new(ICON_VIEW_LIST)
    }
    /// Grid view icon
    pub fn grid_view() -> Self {
        Self::new(ICON_VIEW_MODULE)
    }
    /// Clock/schedule icon
    pub fn clock() -> Self {
        Self::new(ICON_SCHEDULE)
    }
    /// Calendar/event icon
    pub fn calendar() -> Self {
        Self::new(ICON_EVENT)
    }
    /// Today icon
    pub fn today() -> Self {
        Self::new(ICON_TODAY)
    }
}

/// Bundle for spawning a Material Icon as a UI element
///
/// This creates an image element that renders the icon using embedded bitmap data.
#[derive(Bundle, Default)]
pub struct IconBundle {
    /// The icon to display
    pub icon: MaterialIcon,
    /// Icon style configuration
    pub style: IconStyle,
    /// Text span for rendering (will be populated by the icon system)
    pub text: Text,
    /// UI node configuration
    pub node: Node,
}

impl IconBundle {
    /// Create a new icon bundle
    pub fn new(icon: MaterialIcon) -> Self {
        Self {
            icon,
            text: Text::new(icon.as_str()),
            ..default()
        }
    }

    /// Create with a specific style
    pub fn with_style(mut self, style: IconStyle) -> Self {
        self.style = style;
        self
    }

    /// Create with a specific size
    pub fn with_size(mut self, size: f32) -> Self {
        self.style = self.style.with_size(size);
        self.node.width = Val::Px(size);
        self.node.height = Val::Px(size);
        self
    }

    /// Create with a specific color
    pub fn with_color(mut self, color: Color) -> Self {
        self.style = self.style.with_color(color);
        self
    }
}

/// Plugin for the Material Icons system
pub struct IconPlugin;

impl Plugin for IconPlugin {
    fn build(&self, app: &mut App) {
        // Run after most UI construction systems so icons become renderable
        // in the same frame they're spawned (important for UIs that rebuild on resize).
        app.add_systems(PostUpdate, sync_icon_render_components);
    }
}

/// Keep icon entities renderable by ensuring required text components exist.
///
/// Many widgets spawn icons as `(MaterialIcon, IconStyle)` only; Bevy UI renders
/// text using `Text` + `TextFont` (and optionally `TextColor`). This system
/// bridges that gap and also keeps size/color in sync when `IconStyle` changes.
fn sync_icon_render_components(
    icon_font: Option<Res<MaterialIconFont>>,
    mut fonts: ResMut<Assets<Font>>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &MaterialIcon,
        &IconStyle,
        Option<&Node>,
        Option<&mut Text>,
        Option<&mut TextFont>,
        Option<&mut TextColor>,
    )>,
) {
    // Ensure the icon font resource exists even if the user forgets to add
    // `MaterialIconsPlugin` (or if startup order changes).
    // Note: Despite the name, this handles the legacy font-based icon system.
    let mut created_icon_font_this_frame = false;
    let ensured_font_handle: Option<Handle<Font>> = match icon_font.as_ref() {
        Some(font) => Some(font.0.clone()),
        None => {
            let font = Font::try_from_bytes(EMBEDDED_MATERIAL_SYMBOLS_FONT.to_vec())
                .expect("Failed to load embedded icon font (legacy compatibility)");
            let font_handle = fonts.add(font);
            commands.insert_resource(MaterialIconFont(font_handle.clone()));
            created_icon_font_this_frame = true;
            Some(font_handle)
        }
    };

    let icon_font_changed =
        created_icon_font_this_frame || icon_font.as_ref().is_some_and(|font| font.is_changed());

    for (entity, icon, style, node, text, text_font, text_color) in query.iter_mut() {
        let desired_text = Text::new(icon.as_str());
        let desired_size = style.effective_size();

        // Fast path: skip entities that are already fully configured.
        // We still re-run when the icon font resource changes (e.g. becomes available).
        // Note: New code should use embedded bitmaps from MaterialIcon component.
        if !icon_font_changed && node.is_some() && text.is_some() {
            let has_text_font = text_font.is_some();
            let font_matches = match (&ensured_font_handle, &text_font) {
                (Some(expected), Some(current)) => current.font == *expected,
                // If we don't have an expected font (shouldn't happen), don't block the fast path.
                (None, Some(_)) => true,
                _ => false,
            };

            if has_text_font && font_matches {
                // Color can be optional; if style has no color we don't enforce TextColor.
                if style.color.is_none() || text_color.is_some() {
                    continue;
                }
            }
        }

        // UI text entities need a `Node` component to participate in layout and render.
        // Many widgets spawn icons as `(MaterialIcon, IconStyle)` only.
        if node.is_none() {
            commands.entity(entity).insert(Node {
                width: Val::Px(desired_size),
                height: Val::Px(desired_size),
                ..default()
            });
        }

        match text {
            Some(mut text) => {
                *text = desired_text;
            }
            None => {
                commands.entity(entity).insert(desired_text);
            }
        }

        match text_font {
            Some(mut text_font) => {
                if let Some(icon_font) = &ensured_font_handle {
                    text_font.font = icon_font.clone();
                }
                text_font.font_size = desired_size;
            }
            None => {
                if let Some(icon_font) = &ensured_font_handle {
                    commands.entity(entity).insert(TextFont {
                        font: icon_font.clone(),
                        font_size: desired_size,
                        ..default()
                    });
                }
            }
        }

        if let Some(color) = style.color {
            match text_color {
                Some(mut text_color) => text_color.0 = color,
                None => {
                    commands.entity(entity).insert(TextColor(color));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_creation() {
        let icon = MaterialIcon::home();
        assert_eq!(icon.codepoint, ICON_HOME);
        assert_eq!(icon.as_str(), ICON_HOME.to_string());
    }

    #[test]
    fn test_icon_from_name() {
        let icon = MaterialIcon::from_name("settings").unwrap();
        assert_eq!(icon.codepoint, ICON_SETTINGS);

        let none = MaterialIcon::from_name("nonexistent");
        assert!(none.is_none());
    }

    #[test]
    fn test_icon_bundle() {
        let bundle = IconBundle::new(MaterialIcon::search())
            .with_size(24.0)
            .with_color(Color::WHITE);

        assert_eq!(bundle.icon.codepoint, ICON_SEARCH);
        assert_eq!(bundle.style.effective_size(), 24.0);
        assert_eq!(bundle.style.color, Some(Color::WHITE));
    }

    #[test]
    fn test_all_icon_constructors() {
        // Just verify they don't panic and return valid icons
        let icons = [
            MaterialIcon::home(),
            MaterialIcon::menu(),
            MaterialIcon::settings(),
            MaterialIcon::search(),
            MaterialIcon::delete(),
            MaterialIcon::add(),
            MaterialIcon::close(),
            MaterialIcon::check(),
            MaterialIcon::error(),
            MaterialIcon::warning(),
            MaterialIcon::play(),
            MaterialIcon::pause(),
            MaterialIcon::dice(),
            MaterialIcon::shield(),
        ];

        for icon in icons {
            assert!(icon.codepoint as u32 > 0);
            assert!(!icon.as_str().is_empty());
        }
    }
}
