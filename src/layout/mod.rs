//! Material Design 3 layout components.
//!
//! Layout components are higher-level building blocks (e.g. Scaffold) that help
//! compose apps from Material UI components while remaining compatible with
//! Bevy UI's flexbox model.

mod adaptive_navigation_scaffold;
mod bottom_navigation_scaffold;
mod list_detail_scaffold;
mod modal_drawer_scaffold;
mod navigation_rail_scaffold;
mod permanent_drawer_scaffold;
mod scaffold_types;
mod supporting_panes_scaffold;

pub use permanent_drawer_scaffold::{spawn_permanent_drawer_scaffold, PermanentDrawerScaffold};

pub use scaffold_types::{PaneEntities, PaneTestIds, ScaffoldEntities, ScaffoldTestIds};

pub use bottom_navigation_scaffold::{
    spawn_bottom_navigation_scaffold, spawn_navigation_bar_scaffold, BottomNavigationScaffold,
    NavigationBarScaffold,
};

pub use navigation_rail_scaffold::{spawn_navigation_rail_scaffold, NavigationRailScaffold};

pub use modal_drawer_scaffold::{spawn_modal_drawer_scaffold, ModalDrawerScaffold};

pub use list_detail_scaffold::{spawn_list_detail_scaffold, ListDetailScaffold};

pub use supporting_panes_scaffold::{spawn_supporting_panes_scaffold, SupportingPanesScaffold};

pub use adaptive_navigation_scaffold::{
    spawn_adaptive_navigation_scaffold, spawn_navigation_suite_scaffold,
    AdaptiveNavigationScaffold, NavigationSuiteScaffold,
};
