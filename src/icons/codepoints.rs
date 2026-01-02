//! Material Design Icon Identifiers
//!
//! This file contains identifiers for Material Design icons.
//! Icons are embedded as bitmaps from the `google-material-design-icons-bin` crate.
//!
//! Note: These constants may reference legacy codepoint values but icons are now
//! rendered as embedded bitmaps, not font glyphs.
//! Icon reference: <https://fonts.google.com/icons>

// Navigation Icons
/// Home icon codepoint
pub const ICON_HOME: char = '\u{E88A}';
/// Menu icon codepoint
pub const ICON_MENU: char = '\u{E5D2}';
/// More vert icon codepoint (three dots vertical)
pub const ICON_MORE_VERT: char = '\u{E5D4}';
/// More horiz icon codepoint (three dots horizontal)
pub const ICON_MORE_HORIZ: char = '\u{E5D3}';
/// Arrow back icon codepoint
pub const ICON_ARROW_BACK: char = '\u{E5C4}';
/// Arrow forward icon codepoint
pub const ICON_ARROW_FORWARD: char = '\u{E5C8}';
/// Arrow upward icon codepoint
pub const ICON_ARROW_UPWARD: char = '\u{E5D8}';
/// Arrow downward icon codepoint
pub const ICON_ARROW_DOWNWARD: char = '\u{E5DB}';
/// Close icon codepoint
pub const ICON_CLOSE: char = '\u{E5CD}';
/// Check icon codepoint
pub const ICON_CHECK: char = '\u{E5CA}';
/// Expand more icon codepoint (chevron down)
pub const ICON_EXPAND_MORE: char = '\u{E5CF}';
/// Expand less icon codepoint (chevron up)
pub const ICON_EXPAND_LESS: char = '\u{E5CE}';
/// Chevron left icon codepoint
pub const ICON_CHEVRON_LEFT: char = '\u{E5CB}';
/// Chevron right icon codepoint
pub const ICON_CHEVRON_RIGHT: char = '\u{E5CC}';

// Action Icons
/// Add icon codepoint (plus)
pub const ICON_ADD: char = '\u{E145}';
/// Remove icon codepoint (minus)
pub const ICON_REMOVE: char = '\u{E15B}';
/// Delete icon codepoint (trash)
pub const ICON_DELETE: char = '\u{E872}';
/// Edit icon codepoint (pencil)
pub const ICON_EDIT: char = '\u{E3C9}';
/// Save icon codepoint (floppy disk)
pub const ICON_SAVE: char = '\u{E161}';
/// Search icon codepoint
pub const ICON_SEARCH: char = '\u{E8B6}';
/// Refresh icon codepoint
pub const ICON_REFRESH: char = '\u{E5D5}';
/// Settings icon codepoint (gear)
pub const ICON_SETTINGS: char = '\u{E8B8}';
/// Help icon codepoint (question mark circle)
pub const ICON_HELP: char = '\u{E887}';
/// Info icon codepoint (info circle)
pub const ICON_INFO: char = '\u{E88E}';
/// Share icon codepoint
pub const ICON_SHARE: char = '\u{E80D}';
/// Download icon codepoint
pub const ICON_DOWNLOAD: char = '\u{F090}';
/// Upload icon codepoint
pub const ICON_UPLOAD: char = '\u{F09B}';
/// Print icon codepoint
pub const ICON_PRINT: char = '\u{E8AD}';
/// Copy icon codepoint
pub const ICON_CONTENT_COPY: char = '\u{E14D}';
/// Paste icon codepoint
pub const ICON_CONTENT_PASTE: char = '\u{E14F}';
/// Cut icon codepoint
pub const ICON_CONTENT_CUT: char = '\u{E14E}';
/// Undo icon codepoint
pub const ICON_UNDO: char = '\u{E166}';
/// Redo icon codepoint
pub const ICON_REDO: char = '\u{E15A}';

// Toggle Icons
/// Checkbox checked icon
pub const ICON_CHECK_BOX: char = '\u{E834}';
/// Checkbox unchecked icon
pub const ICON_CHECK_BOX_OUTLINE_BLANK: char = '\u{E835}';
/// Radio button checked icon
pub const ICON_RADIO_BUTTON_CHECKED: char = '\u{E837}';
/// Radio button unchecked icon
pub const ICON_RADIO_BUTTON_UNCHECKED: char = '\u{E836}';
/// Toggle on icon
pub const ICON_TOGGLE_ON: char = '\u{E9F6}';
/// Toggle off icon
pub const ICON_TOGGLE_OFF: char = '\u{E9F5}';
/// Star filled icon
pub const ICON_STAR: char = '\u{E838}';
/// Star outline icon
pub const ICON_STAR_BORDER: char = '\u{E83A}';
/// Favorite filled icon (heart)
pub const ICON_FAVORITE: char = '\u{E87D}';
/// Favorite outline icon
pub const ICON_FAVORITE_BORDER: char = '\u{E87E}';
/// Visibility icon (eye)
pub const ICON_VISIBILITY: char = '\u{E8F4}';
/// Visibility off icon
pub const ICON_VISIBILITY_OFF: char = '\u{E8F5}';

// Alert/Status Icons
/// Error icon codepoint
pub const ICON_ERROR: char = '\u{E000}';
/// Warning icon codepoint
pub const ICON_WARNING: char = '\u{E002}';
/// Check circle icon codepoint (success)
pub const ICON_CHECK_CIRCLE: char = '\u{E86C}';
/// Cancel icon codepoint
pub const ICON_CANCEL: char = '\u{E5C9}';
/// Block icon codepoint (prohibited)
pub const ICON_BLOCK: char = '\u{E14B}';
/// Notification icon codepoint (bell)
pub const ICON_NOTIFICATIONS: char = '\u{E7F4}';
/// Notification off icon
pub const ICON_NOTIFICATIONS_OFF: char = '\u{E7F6}';

// Content Icons
/// Folder icon codepoint
pub const ICON_FOLDER: char = '\u{E2C7}';
/// Folder open icon codepoint
pub const ICON_FOLDER_OPEN: char = '\u{E2C8}';
/// File icon codepoint (document)
pub const ICON_DESCRIPTION: char = '\u{E873}';
/// Image icon codepoint
pub const ICON_IMAGE: char = '\u{E3F4}';
/// Video icon codepoint
pub const ICON_VIDEOCAM: char = '\u{E04B}';
/// Music icon codepoint
pub const ICON_MUSIC_NOTE: char = '\u{E405}';
/// Link icon codepoint
pub const ICON_LINK: char = '\u{E157}';
/// Attachment icon codepoint
pub const ICON_ATTACH_FILE: char = '\u{E226}';

// Person/Account Icons
/// Person icon codepoint
pub const ICON_PERSON: char = '\u{E7FD}';
/// People icon codepoint (group)
pub const ICON_GROUP: char = '\u{E7EF}';
/// Account circle icon
pub const ICON_ACCOUNT_CIRCLE: char = '\u{E853}';
/// Person add icon
pub const ICON_PERSON_ADD: char = '\u{E7FE}';
/// Login icon
pub const ICON_LOGIN: char = '\u{EA77}';
/// Logout icon
pub const ICON_LOGOUT: char = '\u{E9BA}';

// Communication Icons
/// Email icon codepoint
pub const ICON_EMAIL: char = '\u{E0BE}';
/// Chat icon codepoint
pub const ICON_CHAT: char = '\u{E0B7}';
/// Message icon codepoint
pub const ICON_MESSAGE: char = '\u{E0C9}';
/// Phone icon codepoint
pub const ICON_PHONE: char = '\u{E0CD}';
/// Send icon codepoint
pub const ICON_SEND: char = '\u{E163}';

// Media Control Icons
/// Play arrow icon codepoint
pub const ICON_PLAY_ARROW: char = '\u{E037}';
/// Pause icon codepoint
pub const ICON_PAUSE: char = '\u{E034}';
/// Stop icon codepoint
pub const ICON_STOP: char = '\u{E047}';
/// Skip next icon codepoint
pub const ICON_SKIP_NEXT: char = '\u{E044}';
/// Skip previous icon codepoint
pub const ICON_SKIP_PREVIOUS: char = '\u{E045}';
/// Fast forward icon codepoint
pub const ICON_FAST_FORWARD: char = '\u{E01F}';
/// Fast rewind icon codepoint
pub const ICON_FAST_REWIND: char = '\u{E020}';
/// Replay icon codepoint
pub const ICON_REPLAY: char = '\u{E042}';
/// Shuffle icon codepoint
pub const ICON_SHUFFLE: char = '\u{E043}';
/// Repeat icon codepoint
pub const ICON_REPEAT: char = '\u{E040}';
/// Volume up icon codepoint
pub const ICON_VOLUME_UP: char = '\u{E050}';
/// Volume down icon codepoint
pub const ICON_VOLUME_DOWN: char = '\u{E04D}';
/// Volume mute icon codepoint
pub const ICON_VOLUME_MUTE: char = '\u{E04E}';
/// Volume off icon codepoint
pub const ICON_VOLUME_OFF: char = '\u{E04F}';

// Device Icons
/// Smartphone icon
pub const ICON_SMARTPHONE: char = '\u{E32C}';
/// Tablet icon
pub const ICON_TABLET: char = '\u{E32F}';
/// Laptop icon
pub const ICON_LAPTOP: char = '\u{E31E}';
/// Desktop icon
pub const ICON_DESKTOP_WINDOWS: char = '\u{E30C}';
/// Keyboard icon
pub const ICON_KEYBOARD: char = '\u{E312}';
/// Mouse icon
pub const ICON_MOUSE: char = '\u{E323}';
/// Gamepad icon
pub const ICON_GAMEPAD: char = '\u{E30F}';
/// Wifi icon
pub const ICON_WIFI: char = '\u{E63E}';
/// Bluetooth icon
pub const ICON_BLUETOOTH: char = '\u{E1A7}';
/// Battery full icon
pub const ICON_BATTERY_FULL: char = '\u{E1A4}';
/// Battery alert icon
pub const ICON_BATTERY_ALERT: char = '\u{E19C}';

// Editor/Formatting Icons
/// Format bold icon
pub const ICON_FORMAT_BOLD: char = '\u{E238}';
/// Format italic icon
pub const ICON_FORMAT_ITALIC: char = '\u{E23F}';
/// Format underline icon
pub const ICON_FORMAT_UNDERLINED: char = '\u{E249}';
/// Format list bulleted icon
pub const ICON_FORMAT_LIST_BULLETED: char = '\u{E241}';
/// Format list numbered icon
pub const ICON_FORMAT_LIST_NUMBERED: char = '\u{E242}';
/// Format align left icon
pub const ICON_FORMAT_ALIGN_LEFT: char = '\u{E236}';
/// Format align center icon
pub const ICON_FORMAT_ALIGN_CENTER: char = '\u{E234}';
/// Format align right icon
pub const ICON_FORMAT_ALIGN_RIGHT: char = '\u{E237}';
/// Format color text icon
pub const ICON_FORMAT_COLOR_TEXT: char = '\u{E23C}';
/// Format color fill icon
pub const ICON_FORMAT_COLOR_FILL: char = '\u{E23A}';

// Map/Location Icons
/// Place icon (location pin)
pub const ICON_PLACE: char = '\u{E55F}';
/// My location icon
pub const ICON_MY_LOCATION: char = '\u{E55C}';
/// Navigation icon
pub const ICON_NAVIGATION: char = '\u{E55D}';
/// Directions icon
pub const ICON_DIRECTIONS: char = '\u{E52E}';
/// Map icon
pub const ICON_MAP: char = '\u{E55B}';

// Game/D&D Specific Icons (using closest matches)
/// Casino icon (dice)
pub const ICON_CASINO: char = '\u{EB40}';
/// Extension icon (puzzle piece - for modules)
pub const ICON_EXTENSION: char = '\u{E87B}';
/// Shield icon (for armor class)
pub const ICON_SHIELD: char = '\u{E8E8}';
/// Sports martial arts icon (for combat)
pub const ICON_SPORTS_MARTIAL_ARTS: char = '\u{EA8C}';
/// Auto fix icon (for magic/spells)
pub const ICON_AUTO_FIX_HIGH: char = '\u{E663}';
/// Lightbulb icon (for inspiration)
pub const ICON_LIGHTBULB: char = '\u{E0F0}';
/// Inventory icon (for inventory/backpack)
pub const ICON_INVENTORY_2: char = '\u{E1A1}';
/// Book icon (for spellbook)
pub const ICON_BOOK: char = '\u{E865}';
/// Psychology icon (for wisdom/intelligence)
pub const ICON_PSYCHOLOGY: char = '\u{EA4A}';
/// Fitness center icon (for strength)
pub const ICON_FITNESS_CENTER: char = '\u{EB43}';
/// Speed icon (for dexterity)
pub const ICON_SPEED: char = '\u{E9E4}';
/// Healing icon (health points)
pub const ICON_HEALING: char = '\u{E3F3}';

// Miscellaneous
/// Language icon (globe)
pub const ICON_LANGUAGE: char = '\u{E894}';
/// Dark mode icon (moon)
pub const ICON_DARK_MODE: char = '\u{E51C}';
/// Light mode icon (sun)
pub const ICON_LIGHT_MODE: char = '\u{E518}';
/// Fullscreen icon
pub const ICON_FULLSCREEN: char = '\u{E5D0}';
/// Fullscreen exit icon
pub const ICON_FULLSCREEN_EXIT: char = '\u{E5D1}';
/// Zoom in icon
pub const ICON_ZOOM_IN: char = '\u{E8FF}';
/// Zoom out icon
pub const ICON_ZOOM_OUT: char = '\u{E900}';
/// Lock icon
pub const ICON_LOCK: char = '\u{E897}';
/// Lock open icon
pub const ICON_LOCK_OPEN: char = '\u{E898}';
/// Tune icon (for settings/adjustments)
pub const ICON_TUNE: char = '\u{E429}';
/// Filter list icon
pub const ICON_FILTER_LIST: char = '\u{E152}';
/// Sort icon
pub const ICON_SORT: char = '\u{E164}';
/// Drag handle icon
pub const ICON_DRAG_HANDLE: char = '\u{E25D}';
/// Apps icon (grid)
pub const ICON_APPS: char = '\u{E5C3}';
/// View list icon
pub const ICON_VIEW_LIST: char = '\u{E8EF}';
/// View module icon (grid view)
pub const ICON_VIEW_MODULE: char = '\u{E8F0}';
/// Schedule icon (clock)
pub const ICON_SCHEDULE: char = '\u{E8B5}';
/// Event icon (calendar)
pub const ICON_EVENT: char = '\u{E878}';
/// Today icon
pub const ICON_TODAY: char = '\u{E8DF}';

/// Get an icon codepoint by name
///
/// Returns the Unicode codepoint for the given icon name, or None if not found.
pub fn icon_by_name(name: &str) -> Option<char> {
    match name.trim().to_lowercase().as_str() {
        // Navigation
        "home" => Some(ICON_HOME),
        "menu" => Some(ICON_MENU),
        "more_vert" | "more-vert" => Some(ICON_MORE_VERT),
        "more_horiz" | "more-horiz" => Some(ICON_MORE_HORIZ),
        "arrow_back" | "arrow-back" | "back" => Some(ICON_ARROW_BACK),
        "arrow_forward" | "arrow-forward" | "forward" => Some(ICON_ARROW_FORWARD),
        "arrow_upward" | "arrow-upward" | "up" => Some(ICON_ARROW_UPWARD),
        "arrow_downward" | "arrow-downward" | "down" => Some(ICON_ARROW_DOWNWARD),
        "close" | "x" => Some(ICON_CLOSE),
        "check" | "tick" => Some(ICON_CHECK),
        "expand_more" | "expand-more" | "chevron_down" => Some(ICON_EXPAND_MORE),
        "expand_less" | "expand-less" | "chevron_up" => Some(ICON_EXPAND_LESS),
        "chevron_left" | "chevron-left" => Some(ICON_CHEVRON_LEFT),
        "chevron_right" | "chevron-right" => Some(ICON_CHEVRON_RIGHT),

        // Actions
        "add" | "plus" => Some(ICON_ADD),
        "remove" | "minus" => Some(ICON_REMOVE),
        "delete" | "trash" => Some(ICON_DELETE),
        "edit" | "pencil" => Some(ICON_EDIT),
        "save" => Some(ICON_SAVE),
        "search" => Some(ICON_SEARCH),
        "refresh" => Some(ICON_REFRESH),
        "settings" | "gear" | "cog" => Some(ICON_SETTINGS),
        "help" | "question" => Some(ICON_HELP),
        "info" | "information" => Some(ICON_INFO),
        "share" => Some(ICON_SHARE),
        "download" => Some(ICON_DOWNLOAD),
        "upload" => Some(ICON_UPLOAD),
        "print" => Some(ICON_PRINT),
        "copy" | "content_copy" => Some(ICON_CONTENT_COPY),
        "paste" | "content_paste" => Some(ICON_CONTENT_PASTE),
        "cut" | "content_cut" => Some(ICON_CONTENT_CUT),
        "undo" => Some(ICON_UNDO),
        "redo" => Some(ICON_REDO),

        // Toggles
        "checkbox" | "check_box" => Some(ICON_CHECK_BOX),
        "checkbox_blank" | "check_box_outline_blank" => Some(ICON_CHECK_BOX_OUTLINE_BLANK),
        "radio_checked" | "radio_button_checked" => Some(ICON_RADIO_BUTTON_CHECKED),
        "radio_unchecked" | "radio_button_unchecked" => Some(ICON_RADIO_BUTTON_UNCHECKED),
        "toggle_on" => Some(ICON_TOGGLE_ON),
        "toggle_off" => Some(ICON_TOGGLE_OFF),
        "star" | "star_filled" => Some(ICON_STAR),
        "star_border" | "star_outline" => Some(ICON_STAR_BORDER),
        "favorite" | "heart" | "heart_filled" => Some(ICON_FAVORITE),
        "favorite_border" | "heart_outline" => Some(ICON_FAVORITE_BORDER),
        "visibility" | "eye" | "visible" => Some(ICON_VISIBILITY),
        "visibility_off" | "eye_off" | "hidden" => Some(ICON_VISIBILITY_OFF),

        // Alerts
        "error" => Some(ICON_ERROR),
        "warning" => Some(ICON_WARNING),
        "check_circle" | "success" => Some(ICON_CHECK_CIRCLE),
        "cancel" => Some(ICON_CANCEL),
        "block" | "prohibited" => Some(ICON_BLOCK),
        "notifications" | "bell" => Some(ICON_NOTIFICATIONS),
        "notifications_off" | "bell_off" => Some(ICON_NOTIFICATIONS_OFF),

        // Content
        "folder" => Some(ICON_FOLDER),
        "folder_open" => Some(ICON_FOLDER_OPEN),
        "description" | "document" | "file" => Some(ICON_DESCRIPTION),
        "image" | "photo" => Some(ICON_IMAGE),
        "videocam" | "video" => Some(ICON_VIDEOCAM),
        "music_note" | "music" | "audio" => Some(ICON_MUSIC_NOTE),
        "link" => Some(ICON_LINK),
        "attach_file" | "attachment" => Some(ICON_ATTACH_FILE),

        // Person
        "person" | "user" => Some(ICON_PERSON),
        "group" | "people" | "users" => Some(ICON_GROUP),
        "account_circle" | "avatar" => Some(ICON_ACCOUNT_CIRCLE),
        "person_add" | "add_user" => Some(ICON_PERSON_ADD),
        "login" | "sign_in" => Some(ICON_LOGIN),
        "logout" | "sign_out" => Some(ICON_LOGOUT),

        // Communication
        "email" | "mail" => Some(ICON_EMAIL),
        "chat" => Some(ICON_CHAT),
        "message" | "sms" => Some(ICON_MESSAGE),
        "phone" | "call" => Some(ICON_PHONE),
        "send" => Some(ICON_SEND),

        // Media
        "play" | "play_arrow" => Some(ICON_PLAY_ARROW),
        "pause" => Some(ICON_PAUSE),
        "stop" => Some(ICON_STOP),
        "skip_next" | "next" => Some(ICON_SKIP_NEXT),
        "skip_previous" | "previous" => Some(ICON_SKIP_PREVIOUS),
        "fast_forward" => Some(ICON_FAST_FORWARD),
        "fast_rewind" | "rewind" => Some(ICON_FAST_REWIND),
        "replay" => Some(ICON_REPLAY),
        "shuffle" => Some(ICON_SHUFFLE),
        "repeat" | "loop" => Some(ICON_REPEAT),
        "volume_up" | "volume_high" => Some(ICON_VOLUME_UP),
        "volume_down" | "volume_low" => Some(ICON_VOLUME_DOWN),
        "volume_mute" => Some(ICON_VOLUME_MUTE),
        "volume_off" => Some(ICON_VOLUME_OFF),

        // Devices
        "smartphone" | "mobile" => Some(ICON_SMARTPHONE),
        "tablet" => Some(ICON_TABLET),
        "laptop" => Some(ICON_LAPTOP),
        "desktop" | "computer" => Some(ICON_DESKTOP_WINDOWS),
        "keyboard" => Some(ICON_KEYBOARD),
        "mouse" => Some(ICON_MOUSE),
        "gamepad" | "controller" => Some(ICON_GAMEPAD),
        "wifi" => Some(ICON_WIFI),
        "bluetooth" => Some(ICON_BLUETOOTH),
        "battery_full" | "battery" => Some(ICON_BATTERY_FULL),
        "battery_alert" | "battery_low" => Some(ICON_BATTERY_ALERT),

        // Game/D&D
        "casino" | "dice" => Some(ICON_CASINO),
        "extension" | "puzzle" | "module" => Some(ICON_EXTENSION),
        "shield" | "armor" | "ac" => Some(ICON_SHIELD),
        "combat" | "martial_arts" => Some(ICON_SPORTS_MARTIAL_ARTS),
        "magic" | "spell" | "auto_fix" => Some(ICON_AUTO_FIX_HIGH),
        "lightbulb" | "idea" | "inspiration" => Some(ICON_LIGHTBULB),
        "inventory" | "backpack" => Some(ICON_INVENTORY_2),
        "book" | "spellbook" => Some(ICON_BOOK),
        "psychology" | "mind" | "wisdom" | "intelligence" => Some(ICON_PSYCHOLOGY),
        "fitness" | "strength" | "str" => Some(ICON_FITNESS_CENTER),
        "speed" | "dexterity" | "dex" => Some(ICON_SPEED),
        "healing" | "hp" | "health" => Some(ICON_HEALING),

        // Misc
        "language" | "globe" | "world" => Some(ICON_LANGUAGE),
        "dark_mode" | "moon" | "night" => Some(ICON_DARK_MODE),
        "light_mode" | "sun" | "day" => Some(ICON_LIGHT_MODE),
        "fullscreen" => Some(ICON_FULLSCREEN),
        "fullscreen_exit" => Some(ICON_FULLSCREEN_EXIT),
        "zoom_in" => Some(ICON_ZOOM_IN),
        "zoom_out" => Some(ICON_ZOOM_OUT),
        "lock" | "locked" => Some(ICON_LOCK),
        "lock_open" | "unlocked" => Some(ICON_LOCK_OPEN),
        "tune" | "adjust" => Some(ICON_TUNE),
        "filter" | "filter_list" => Some(ICON_FILTER_LIST),
        "sort" => Some(ICON_SORT),
        "drag" | "drag_handle" => Some(ICON_DRAG_HANDLE),
        "apps" | "grid" => Some(ICON_APPS),
        "view_list" | "list" => Some(ICON_VIEW_LIST),
        "view_module" | "grid_view" => Some(ICON_VIEW_MODULE),
        "schedule" | "clock" | "time" => Some(ICON_SCHEDULE),
        "event" | "calendar" => Some(ICON_EVENT),
        "today" => Some(ICON_TODAY),

        // -----------------------------------------------------------------
        // Reference drawable-name compatibility
        // -----------------------------------------------------------------
        // DatePicker drawables
        "material_ic_calendar_black_24dp" => Some(ICON_EVENT),
        "material_ic_edit_black_24dp" => Some(ICON_EDIT),
        "material_ic_clear_black_24dp" => Some(ICON_CLOSE),
        "material_ic_menu_arrow_down_black_24dp" => Some(ICON_EXPAND_MORE),
        "material_ic_menu_arrow_up_black_24dp" => Some(ICON_EXPAND_LESS),
        "material_ic_keyboard_arrow_left_black_24dp" => Some(ICON_CHEVRON_LEFT),
        "material_ic_keyboard_arrow_right_black_24dp" => Some(ICON_CHEVRON_RIGHT),
        // Some source sets provide these as RTL-flipping aliases; we map to
        // logical previous/next for LTR by default.
        "material_ic_keyboard_arrow_previous_black_24dp" => Some(ICON_CHEVRON_LEFT),
        "material_ic_keyboard_arrow_next_black_24dp" => Some(ICON_CHEVRON_RIGHT),

        // TimePicker drawables
        "ic_keyboard_black_24dp" => Some(ICON_KEYBOARD),
        "ic_clock_black_24dp" => Some(ICON_SCHEDULE),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_by_name() {
        assert_eq!(icon_by_name("home"), Some(ICON_HOME));
        assert_eq!(icon_by_name("Home"), Some(ICON_HOME));
        assert_eq!(icon_by_name("settings"), Some(ICON_SETTINGS));
        assert_eq!(icon_by_name("gear"), Some(ICON_SETTINGS));

        // Drawable resource names (name part only)
        assert_eq!(
            icon_by_name("material_ic_clear_black_24dp"),
            Some(ICON_CLOSE)
        );
        assert_eq!(
            icon_by_name("material_ic_calendar_black_24dp"),
            Some(ICON_EVENT)
        );

        assert_eq!(icon_by_name("nonexistent"), None);
    }

    #[test]
    fn test_icon_codepoints_are_valid_unicode() {
        // Verify all codepoints are in the Private Use Area or standard Unicode
        let icons = [
            ICON_HOME,
            ICON_MENU,
            ICON_CLOSE,
            ICON_CHECK,
            ICON_SETTINGS,
            ICON_SEARCH,
            ICON_DELETE,
            ICON_ADD,
        ];
        for icon in icons {
            assert!(icon as u32 > 0, "Icon codepoint should be non-zero");
            assert!(
                icon as u32 <= 0x10FFFF,
                "Icon codepoint should be valid Unicode"
            );
        }
    }
}
