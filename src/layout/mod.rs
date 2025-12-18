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

mod adaptive_navigation_scaffold;
mod app_bar_inset;
mod bottom_navigation_scaffold;
mod list_detail_scaffold;
mod modal_drawer_scaffold;
mod navigation_rail_scaffold;
mod permanent_drawer_scaffold;
mod scaffold_types;
mod supporting_panes_scaffold;

pub mod navigation {
    //! Navigation-focused scaffolds (bottom nav, rail, drawers, adaptive).
    pub use super::adaptive_navigation_scaffold::{
        spawn_adaptive_navigation_scaffold, spawn_navigation_suite_scaffold,
        AdaptiveNavigationScaffold, NavigationSuiteScaffold,
    };
    pub use super::bottom_navigation_scaffold::{
        spawn_bottom_navigation_scaffold, spawn_navigation_bar_scaffold, BottomNavigationScaffold,
        NavigationBarScaffold,
    };
    pub use super::modal_drawer_scaffold::{spawn_modal_drawer_scaffold, ModalDrawerScaffold};
    pub use super::navigation_rail_scaffold::{
        spawn_navigation_rail_scaffold, NavigationRailScaffold,
    };
    pub use super::permanent_drawer_scaffold::{
        spawn_permanent_drawer_scaffold, PermanentDrawerScaffold,
    };
}

pub mod panes {
    //! Pane-based scaffolds for split and supporting content layouts.
    pub use super::list_detail_scaffold::{spawn_list_detail_scaffold, ListDetailScaffold};
    pub use super::supporting_panes_scaffold::{
        spawn_supporting_panes_scaffold, SupportingPanesScaffold,
    };
}

pub mod app_bar {
    //! App bar layout helpers (offset/inset calculations).
    pub use super::app_bar_inset::{apply_app_bar_inset, AppBarOffsetConfig};
}

pub mod types {
    //! Shared scaffold entity handles and test identifiers.
    pub use super::scaffold_types::{PaneEntities, PaneTestIds, ScaffoldEntities, ScaffoldTestIds};
}

pub use app_bar::*;
pub use navigation::*;
pub use panes::*;
pub use types::*;
