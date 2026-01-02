# Changelog

## 0.2.4 (2026-01-02)

### Breaking Changes
- **Text Field API**: The trait method `spawn_text_field_with` has been removed. Use the standalone function `spawn_text_field_control_with` instead.
  - **Migration**: See [docs/MIGRATION_v0.2.4.md](docs/MIGRATION_v0.2.4.md) for detailed migration steps
- **Color Palette Generation**: Primary palette now uses `max(48.0, seed_chroma)` instead of fixed `48.0`, preserving highly chromatic seed colors (>48 chroma) while maintaining MD3's minimum vibrancy threshold. This may produce different primary colors for highly saturated seeds.

### Code Quality & Cleanup
- **Removed Legacy Code**: Deleted unused internal HCT implementation files (hct.rs, math.rs) totaling ~1,051 lines - now exclusively using external `hct-cam16` crate
- **Centralized Date Validation**: Removed redundant `DateInputPattern::is_valid_complete_basic()` method - all date validation now uses `Date::is_valid()` with proper leap year and days-in-month checks
- **Removed Dead Code**: Cleaned up unused `CalendarPresenter` placeholder struct and `DatePickerDialog.picker` field
- **MAX_ANCESTOR_DEPTH Constant**: Replaced magic number 32 with named constant across scroll.rs, list.rs, and app_bar.rs for better code discoverability

### Production Readiness
- **CurrentDate Resource**: Added `CurrentDate` resource pattern for production date handling in date picker
  - Date picker systems now check for `CurrentDate` resource before falling back to placeholder
  - Comprehensive documentation with integration examples for `chrono` and `time` crates
- **Enhanced Documentation**: Added clear warnings about `Date::today()` placeholder implementation

### Internationalization (i18n)
- **Comprehensive i18n Implementation**: Full internationalization support across all showcase views with 95+ translation keys
- **Multi-Language Support**: 7 languages fully supported (en-US, es-ES, fr-FR, de-DE, ja-JP, zh-CN, he-IL)
- **Multi-Script Font System**: Automatic font switching for Latin, CJK (Chinese/Japanese/Korean), and Hebrew scripts
- **i18n Helper Functions**: Custom spawn functions in `i18n_helpers.rs` for localized component labels
- **Documentation**: Three comprehensive guides added:
  - `docs/INTERNATIONALIZATION.md` - Complete i18n architecture and usage guide
  - `docs/I18N_IMPLEMENTATION_REVIEW.md` - Implementation details and patterns
  - `docs/I18N_QUICK_REFERENCE.md` - Quick reference for developers

### Performance & Benchmarking
- **Expanded Benchmark Coverage**: Added benchmarks for 7 additional components:
  - FAB (Floating Action Button)
  - IconButton
  - Card
  - List
  - LoadingIndicator
  - SearchBar
  - Divider
- **System Benchmarks**: Added entity spawning benchmarks for all major components (10-1000 entities)
- **Performance Verification**: All benchmarks pass with optimal metrics:
  - Component creation: sub-nanosecond (~400-600 ps)
  - Entity spawning: ~290-305 µs for 100 entities, scales linearly
  - System operations: <1 µs for most updates
  - No performance degradation from i18n implementation
- **CI/CD Integration**: Automated benchmark workflow with:
  - Runs on every push to main and v0.2.4
  - Performance tracking across commits
  - Regression alerts (>200% threshold)
  - Per-branch result storage
  - Badge display on README

### Showcase Application Updates
- **All Views Localized**: Updated 14 showcase views with i18n support:
  - Buttons, Checkboxes, Switches, Radios, Chips, FAB, Progress, Sliders
  - Cards, Dividers, Lists, Search, Loading Indicators, Section Headers
- **Code Examples Enhanced**: Updated examples to demonstrate both simple API and i18n patterns
- **Translation Files Complete**: All `.mui_lang` files updated with comprehensive key coverage
- **Fixed Missed Labels**: Corrected hardcoded labels in sliders, lists, cards, dividers, and other components

### Developer Experience
- **Translation Key Convention**: Established hierarchical naming pattern `showcase.{view}.{element}`
- **Best Practices Documentation**: Clear guidelines for adding i18n to applications
- **Custom Component Patterns**: Demonstrated manual construction for complex i18n scenarios
- **Font System Architecture**: Explained `NeedsInternationalFont` marker and automatic font switching

### Quality & Maintenance
- **Performance Badge**: Added benchmark status badge to README
- **Benchmark Fixtures**: Fixed import issues and API calls in benchmark code
- **Documentation Coverage**: Three new comprehensive i18n guides totaling ~400+ lines
- **Code Quality**: All changes maintain existing code quality standards

## 0.2.3 (2025-12-27)

- App Bar: add `spawn_top_app_bar_with_right_content` and `SpawnTopAppBarWithRightContentChild` for a proper MD3 Top App Bar with a right-side custom content slot.
- Showcase: update the App Bars view to demonstrate the new API.

## 0.2.1 (2025-12-17)

- Examples: `all_icon_buttons` now shows tooltips with icon names (when available).
- Maintenance: address minor unused-variable/dead-code warnings in a few components.

## 0.2.0 (2025-12-16)

- Text field: add `auto_focus` support and builder API.
- Text field: optional clipboard integration via `clipboard` feature.
- Text field: add standalone spawn helpers (`spawn_text_field_control`, `spawn_text_field_control_with`).
- Slider: add standalone spawn helpers (`spawn_slider_control`, `spawn_slider_control_with`).
- Slider: improve orientation/direction rendering behavior.
- Docs: update Text Field and Slider component documentation.
