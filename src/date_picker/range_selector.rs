//! Date range selection logic

use super::types::{Date, DateSelection};

/// Trait for date selection strategies
pub trait DateSelector: Send + Sync {
    fn selection(&self) -> Option<DateSelection>;
    fn set_selection(&mut self, selection: DateSelection);
    fn clear(&mut self);
    fn clone_box(&self) -> Box<dyn DateSelector>;
}

impl Clone for Box<dyn DateSelector> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Single date selector
#[derive(Debug, Clone)]
pub struct SingleDateSelector {
    date: Option<Date>,
}

impl SingleDateSelector {
    pub fn new() -> Self {
        Self { date: None }
    }

    pub fn with_date(date: Date) -> Self {
        Self { date: Some(date) }
    }
}

impl Default for SingleDateSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl DateSelector for SingleDateSelector {
    fn selection(&self) -> Option<DateSelection> {
        self.date.map(DateSelection::Single)
    }

    fn set_selection(&mut self, selection: DateSelection) {
        match selection {
            DateSelection::Single(date) => {
                self.date = Some(date);
            }
            DateSelection::Range { start, .. } => {
                // If given a range, just use the start date
                self.date = Some(start);
            }
        }
    }

    fn clear(&mut self) {
        self.date = None;
    }

    fn clone_box(&self) -> Box<dyn DateSelector> {
        Box::new(self.clone())
    }
}

/// Range date selector
#[derive(Debug, Clone)]
pub struct RangeDateSelector {
    start: Option<Date>,
    end: Option<Date>,
}

impl RangeDateSelector {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn with_range(start: Date, end: Option<Date>) -> Self {
        Self {
            start: Some(start),
            end,
        }
    }

    pub fn start(&self) -> Option<Date> {
        self.start
    }

    pub fn end(&self) -> Option<Date> {
        self.end
    }

    pub fn is_complete(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }
}

impl Default for RangeDateSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl DateSelector for RangeDateSelector {
    fn selection(&self) -> Option<DateSelection> {
        self.start.map(|start| DateSelection::Range {
            start,
            end: self.end,
        })
    }

    fn set_selection(&mut self, selection: DateSelection) {
        match selection {
            DateSelection::Single(date) => {
                // Start a new range
                self.start = Some(date);
                self.end = None;
            }
            DateSelection::Range { start, end } => {
                self.start = Some(start);
                self.end = end;
            }
        }
    }

    fn clear(&mut self) {
        self.start = None;
        self.end = None;
    }

    fn clone_box(&self) -> Box<dyn DateSelector> {
        Box::new(self.clone())
    }
}
