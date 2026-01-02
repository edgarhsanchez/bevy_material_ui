# DateTime Picker Refactor Plan

## Executive Summary

The current `datetime_picker.rs` implementation combines date and time selection in a single component, which doesn't match Material Design 3 specifications. This document outlines the complete refactoring plan to align with the reference Material Components behavior.

## Critical Issues Identified

### 1. **Architectural Mismatch**
- ❌ Current: Combined date+time picker in one dialog
- ✅ Required: Separate MaterialDatePicker and MaterialTimePicker components
- **Impact**: Violates Material Design 3 specifications, confusing UX

### 2. **Missing Input Modes**
-  Date Picker needs: CALENDAR ↔ TEXT INPUT modes
-  Time Picker needs: CLOCK ↔ KEYBOARD modes
- **Impact**: Poor accessibility, no power-user workflows

### 3. **Time Picker Uses Buttons Instead of Clock Face**
- ❌ Current: +/- increment buttons
- ✅ Required: Radial clock face with draggable hand
- **Impact**: Non-standard UI, poor touch interaction

### 4. **No Date Range Selection**
- ❌ Current: Single date only
- ✅ Required: Single date OR date range selection
- **Impact**: Cannot build apps requiring date range (booking, analytics, etc.)

## Refactoring Strategy

### Phase 1: Foundation (Week 1)
**Goal**: Create separate, properly architected components

#### 1.1 Create New File Structure
```
src/
  date_picker/
    mod.rs              # Public API
    calendar.rs         # Calendar presenter
    text_input.rs       # Text input presenter  
    constraints.rs      # Date validation
    range_selector.rs   # Date range logic
  time_picker/
    mod.rs              # Public API
    clock.rs            # Clock face presenter
    keyboard.rs         # Keyboard input presenter
    format.rs           # 12/24H handling
  datetime_picker.rs    # DEPRECATED (compatibility shim)
```

#### 1.2 Define Core Types
```rust
// Date Picker
pub enum DatePickerMode { Single, Range }
pub enum DateInputMode { Calendar, Text }
pub struct MaterialDatePicker { /* ... */ }
pub trait DateSelector: Send + Sync {
    fn selection(&self) -> Option<DateSelection>;
    fn set_selection(&mut self, selection: DateSelection);
}

// Time Picker  
pub enum TimeInputMode { Clock, Keyboard }
pub struct MaterialTimePicker { /* ... */ }
pub struct ClockFace { hour: u8, minute: u8, mode: ClockMode }
```

#### 1.3 Implement DateSelector Trait
```rust
pub enum DateSelection {
    Single(Date),
    Range { start: Date, end: Option<Date> },
}

pub struct SingleDateSelector(Option<Date>);
pub struct RangeDateSelector { start: Option<Date>, end: Option<Date> };
```

### Phase 2: Date Picker (Week 1-2)
**Goal**: Fully functional date picker matching Material Design 3

#### 2.1 Calendar Presenter
- ✅ Month grid with proper day layout
- ✅ Previous/next month navigation
- ✅ Year selector (scrollable grid)
- ✅ Today highlight
- ✅ Selection states (selected, range start, range end, range middle)
- ✅ Disabled dates (out of bounds, custom validators)

#### 2.2 Text Input Presenter
- ✅ Formatted text input (MM/DD/YYYY, etc.)
- ✅ Real-time validation
- ✅ Error states and messages
- ✅ Format hints (placeholder text)
- ✅ Parse multiple date formats

#### 2.3 Calendar Constraints
```rust
pub struct CalendarConstraints {
    pub start: Month,           // First selectable month
    pub end: Month,             // Last selectable month
    pub opening: Month,         // Initial display month
    pub validator: Box<dyn DateValidator>,
}

pub trait DateValidator: Send + Sync {
    fn is_valid(&self, date: Date) -> bool;
}
```

#### 2.4 Range Selection
- Visual feedback: start date (outlined), end date (outlined), middle dates (filled)
- Logic: First click sets start, second click sets end
- Validation: End date must be >= start date
- Clear/reset functionality

### Phase 3: Time Picker (Week 2-3)
**Goal**: Radial clock face with full feature parity

#### 3.1 Clock Face Layout
```
     12/00
   11    01
  10      02
  09      03
   08    04
     06/18
     
Inner ring (0-11): for 24H format first half
Outer ring (12-23): for 24H format second half
Single ring (1-12): for 12H format
```

#### 3.2 Clock Interaction
- Touch/drag to set hour or minute
- Automatic mode switching: Select hour → auto-switch to minute
- Haptic feedback on value change
- Smooth rotation animation
- Snap to valid values (5-minute increments by default)

#### 3.3 Keyboard Input Mode
```rust
struct TimeKeyboardInput {
    hour_field: TextInput,
    minute_field: TextInput,
    active_field: TimeField,
}
```
- Two text fields (hour, minute)
- Tab navigation
- Format validation (12H: 1-12, 24H: 0-23)
- Real-time error display

#### 3.4 Format Handling
- Detect system default (Locale::is_24_hour_format())
- 12H mode: Show AM/PM toggle, single clock ring
- 24H mode: Dual-level clock (inner/outer rings)
- Allow manual override

### Phase 4: UI/UX Polish (Week 3)
**Goal**: Production-ready experience

#### 4.1 Material Design 3 Theming
- Proper dialog shape (ExtraLarge corner radius)
- Surface elevation (surface_container_high)
- State layers (hover, pressed, focus)
- Color roles (primary, on_primary, surface, on_surface_variant)

#### 4.2 Animations
- Mode toggle fade transition (200ms)
- Month swipe animation (300ms)
- Clock hand rotation (150ms with overshoot)
- Year selector scroll spring

#### 4.3 Accessibility
- Content descriptions for all buttons
- Live region announcements: "Selected January 15, 2025"
- Keyboard navigation (Tab, Arrow keys, Enter, Escape)
- Screen reader support (announce selection changes)

#### 4.4 Header Improvements
**Date Picker Header:**
```
┌────────────────────────────────────┐
│ Select date              [📝][✕]  │  
│                                    │
│ Wed, Jan 15, 2025                  │  <- Selection preview
└────────────────────────────────────┘
```

**Time Picker Header:**
```
┌────────────────────────────────────┐
│ Select time              [⌨][✕]   │
│                                    │
│    [10] : [30]    [AM][PM]         │  <- Interactive chips
└────────────────────────────────────┘
```

### Phase 5: Advanced Features (Week 4)
**Goal**: Enterprise-grade features

#### 5.1 Day View Decorator
```rust
pub trait DayViewDecorator: Send + Sync {
    fn decorate(&self, date: Date) -> DayDecoration;
}

pub struct DayDecoration {
    pub badge: Option<String>,
    pub background: Option<Color>,
    pub border: Option<(Color, f32)>,
}
```

#### 5.2 Custom Validators
```rust
// Example: Block weekends
pub struct NoWeekendsValidator;
impl DateValidator for NoWeekendsValidator {
    fn is_valid(&self, date: Date) -> bool {
        let wd = weekday_for_date(date);
        wd != Weekday::Sat && wd != Weekday::Sun
    }
}

// Example: Only allow specific dates
pub struct AllowListValidator(Vec<Date>);
```

#### 5.3 Responsive Layouts
- Portrait: Standard dialog (360px width)
- Landscape: Side-by-side layout (calendar + time in one view)
- Fullscreen: For small devices (<600px)

#### 5.4 State Preservation
- Save state on window unfocus
- Restore state on app resume
- Handle configuration changes

## Breaking Changes

### API Changes
```rust
// BEFORE (deprecated)
DateTimePickerBuilder::new()
    .date(Date::new(2025, 1, 15))
    .time(10, 30)
    .spawn()

// AFTER (new API)
// Date picker
DatePickerBuilder::new()
    .mode(DatePickerMode::Single)
    .initial_selection(Date::new(2025, 1, 15))
    .spawn()

// Time picker (separate)
TimePickerBuilder::new()
    .initial_time(10, 30)
    .format(TimeFormat::H12)
    .spawn()
```

### Migration Path
1. Keep `datetime_picker.rs` as compatibility shim for 1-2 releases
2. Add deprecation warnings
3. Update all examples to use new API
4. Remove old implementation in next major version

## Testing Strategy

### Unit Tests
- Date math (leap years, month boundaries)
- Time format conversion (12H ↔ 24H)
- Range selection logic
- Validator chains
- Constraint validation

### Integration Tests  
- Mode switching (Calendar ↔ Text, Clock ↔ Keyboard)
- Touch interaction on clock face
- Range selection flow
- Keyboard navigation
- Theme updates

### Visual Tests
- All DateItemStyle states
- Clock hand rotation
- Month transitions
- Range highlighting
- Disabled date rendering

## Performance Considerations

### Calendar Paging
- Render only: current month + 1 previous + 1 next
- Lazy load months on demand
- Reuse cell entities (pooling)

### Clock Face
- Use single mesh for clock background
- Update hand rotation via Transform
- Cache trigonometry calculations

### Text Input
- Debounce validation (150ms)
- Async date parsing (off main thread)

## Documentation Updates

### User Guide
- New "Date Picker" guide with examples
- New "Time Picker" guide with examples
- Migration guide from old API
- Accessibility best practices

### API Documentation
- Comprehensive Rustdoc for all public APIs
- Code examples for common scenarios
- Trait documentation with default implementations

### Developer Guide
- Architecture overview
- Adding custom validators
- Creating decorators
- Theming pickers

## Timeline

| Week | Focus | Deliverables |
|------|-------|--------------|
| 1 | Architecture + Date Calendar | Separate components, calendar mode working |
| 2 | Date Text Input + Time Clock | Full date picker, clock face functional |
| 3 | Time Keyboard + Polish | Full time picker, animations, accessibility |
| 4 | Advanced Features + Docs | Decorators, validators, comprehensive docs |

## Success Criteria

- ✅ Matches reference Material Components behavior
- ✅ All P0 features implemented
- ✅ 90%+ test coverage
- ✅ Comprehensive documentation
- ✅ Zero regressions in existing examples
- ✅ Performance: <16ms frame time on clock interaction

## Open Questions

1. Should we support inline pickers (non-dialog)?
2. Do we need custom animation curves?
3. Should validators be async (for server-side validation)?
4. Do we need multi-month view (show 2-3 months simultaneously)?

## References

- [Material Design 3 Date Picker](https://m3.material.io/components/date-pickers/overview)
- [Material Design 3 Time Picker](https://m3.material.io/components/time-pickers/overview)
- Reference implementation: `MaterialDatePicker.java`
- Reference implementation: `MaterialTimePicker.java`
