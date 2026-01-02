//! Calendar constraints and validation

use super::types::{Date, Month};

/// Trait for validating dates
pub trait DateValidator: Send + Sync {
    fn is_valid(&self, date: Date) -> bool;
    fn clone_box(&self) -> Box<dyn DateValidator>;
}

impl Clone for Box<dyn DateValidator> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Default validator that allows all dates
#[derive(Debug, Clone)]
pub struct AllDatesValidator;

impl DateValidator for AllDatesValidator {
    fn is_valid(&self, _date: Date) -> bool {
        true
    }

    fn clone_box(&self) -> Box<dyn DateValidator> {
        Box::new(self.clone())
    }
}

/// Validator that blocks weekends
#[derive(Debug, Clone)]
pub struct NoWeekendsValidator;

impl DateValidator for NoWeekendsValidator {
    fn is_valid(&self, date: Date) -> bool {
        use super::types::{weekday_for_date, Weekday};
        let wd = weekday_for_date(date);
        wd != Weekday::Sat && wd != Weekday::Sun
    }

    fn clone_box(&self) -> Box<dyn DateValidator> {
        Box::new(self.clone())
    }
}

/// Validator that only allows specific dates
#[derive(Debug, Clone)]
pub struct AllowListValidator {
    allowed: Vec<Date>,
}

impl AllowListValidator {
    pub fn new(allowed: Vec<Date>) -> Self {
        Self { allowed }
    }
}

impl DateValidator for AllowListValidator {
    fn is_valid(&self, date: Date) -> bool {
        self.allowed.contains(&date)
    }

    fn clone_box(&self) -> Box<dyn DateValidator> {
        Box::new(self.clone())
    }
}

/// Validator that blocks specific dates
#[derive(Debug, Clone)]
pub struct BlockListValidator {
    blocked: Vec<Date>,
}

impl BlockListValidator {
    pub fn new(blocked: Vec<Date>) -> Self {
        Self { blocked }
    }
}

impl DateValidator for BlockListValidator {
    fn is_valid(&self, date: Date) -> bool {
        !self.blocked.contains(&date)
    }

    fn clone_box(&self) -> Box<dyn DateValidator> {
        Box::new(self.clone())
    }
}

/// Validator that enforces min/max bounds
#[derive(Debug, Clone)]
pub struct BoundsValidator {
    min: Option<Date>,
    max: Option<Date>,
}

impl BoundsValidator {
    pub fn new(min: Option<Date>, max: Option<Date>) -> Self {
        Self { min, max }
    }
}

impl DateValidator for BoundsValidator {
    fn is_valid(&self, date: Date) -> bool {
        if let Some(min) = self.min {
            if date < min {
                return false;
            }
        }
        if let Some(max) = self.max {
            if date > max {
                return false;
            }
        }
        true
    }

    fn clone_box(&self) -> Box<dyn DateValidator> {
        Box::new(self.clone())
    }
}

/// Composite validator that combines multiple validators
#[derive(Clone)]
pub struct CompositeValidator {
    validators: Vec<Box<dyn DateValidator>>,
}

impl CompositeValidator {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn add_validator(mut self, validator: Box<dyn DateValidator>) -> Self {
        self.validators.push(validator);
        self
    }
}

impl Default for CompositeValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl DateValidator for CompositeValidator {
    fn is_valid(&self, date: Date) -> bool {
        self.validators.iter().all(|v| v.is_valid(date))
    }

    fn clone_box(&self) -> Box<dyn DateValidator> {
        Box::new(CompositeValidator {
            validators: self.validators.iter().map(|v| v.clone_box()).collect(),
        })
    }
}

/// Calendar constraints defining selectable date range
#[derive(Clone)]
pub struct CalendarConstraints {
    /// First selectable month
    pub start: Month,
    /// Last selectable month
    pub end: Month,
    /// Month to display when picker opens
    pub opening: Month,
    /// Custom date validator
    pub validator: Box<dyn DateValidator>,
}

impl std::fmt::Debug for CalendarConstraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CalendarConstraints")
            .field("start", &self.start)
            .field("end", &self.end)
            .field("opening", &self.opening)
            .field("validator", &"<DateValidator>")
            .finish()
    }
}

impl Default for CalendarConstraints {
    fn default() -> Self {
        let current = Month::current();
        Self {
            start: current.add_months(-600), // ~50 years ago
            end: current.add_months(600),    // ~50 years ahead
            opening: current,
            validator: Box::new(AllDatesValidator),
        }
    }
}

impl CalendarConstraints {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_bounds(mut self, start: Month, end: Month) -> Self {
        self.start = start;
        self.end = end;
        self
    }

    pub fn with_opening(mut self, opening: Month) -> Self {
        self.opening = opening;
        self
    }

    pub fn with_validator(mut self, validator: Box<dyn DateValidator>) -> Self {
        self.validator = validator;
        self
    }

    pub fn with_min_max_dates(mut self, min: Option<Date>, max: Option<Date>) -> Self {
        if let Some(min) = min {
            self.start = Month::new(min.year, min.month);
        }
        if let Some(max) = max {
            self.end = Month::new(max.year, max.month);
        }
        self.validator = Box::new(BoundsValidator::new(min, max));
        self
    }

    pub fn is_month_enabled(&self, month: Month) -> bool {
        month >= self.start && month <= self.end
    }
}
