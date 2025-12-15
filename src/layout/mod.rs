//! Material Design 3 layout components.
//!
//! Layout components are higher-level building blocks (e.g. Scaffolds) that
//! compose navigation, content panes, and app bars while remaining compatible
//! with Bevy UI's flexbox model.
//!
//! Modules:
//! - `navigation`: Navigation bar (bottom), navigation rail, navigation drawers
//!   (permanent/modal), and the adaptive "navigation suite" scaffold.
//! - `panes`: List-detail and supporting panes scaffolds.
//! - `app_bar`: Utilities for coordinating top app bar insets.
//! - `types`: Shared entity and test-id groupings for scaffolds.

mod scaffold_types;
mod permanent_drawer_scaffold;
mod navigation_rail_scaffold;
mod bottom_navigation_scaffold;
mod adaptive_navigation_scaffold;
mod modal_drawer_scaffold;
mod list_detail_scaffold;
mod supporting_panes_scaffold;
mod app_bar_inset;

pub mod navigation {
    //! Navigation-focused scaffolds (bottom nav, rail, drawers, adaptive).
    pub use super::adaptive_navigation_scaffold::{
        AdaptiveNavigationScaffold,
        NavigationSuiteScaffold,
        spawn_adaptive_navigation_scaffold,
        spawn_navigation_suite_scaffold,
    };
    pub use super::bottom_navigation_scaffold::{
        BottomNavigationScaffold,
        NavigationBarScaffold,
        spawn_bottom_navigation_scaffold,
        spawn_navigation_bar_scaffold,
    };
    pub use super::modal_drawer_scaffold::{
        ModalDrawerScaffold,
        spawn_modal_drawer_scaffold,
    };
    pub use super::navigation_rail_scaffold::{
        NavigationRailScaffold,
        spawn_navigation_rail_scaffold,
    };
    pub use super::permanent_drawer_scaffold::{
        PermanentDrawerScaffold,
        spawn_permanent_drawer_scaffold,
    };
}

pub mod panes {
    //! Pane-based scaffolds for split and supporting content layouts.
    pub use super::list_detail_scaffold::{
        ListDetailScaffold,
        spawn_list_detail_scaffold,
    };
    pub use super::supporting_panes_scaffold::{
        SupportingPanesScaffold,
        spawn_supporting_panes_scaffold,
    };
}

pub mod app_bar {
    //! App bar layout helpers (offset/inset calculations).
    pub use super::app_bar_inset::{
        AppBarOffsetConfig,
        apply_app_bar_inset,
    };
}

pub mod types {
    //! Shared scaffold entity handles and test identifiers.
    pub use super::scaffold_types::{
        PaneEntities,
        PaneTestIds,
        ScaffoldEntities,
        ScaffoldTestIds,
    };
}

pub use navigation::*;
pub use panes::*;
pub use app_bar::*;
pub use types::*;
