//! Time format utilities

/// Convert 24H hour to 12H format
pub fn hour_24_to_12(hour_24: u8) -> (u8, bool) {
    let is_pm = hour_24 >= 12;
    let hour_12 = if hour_24 == 0 {
        12
    } else if hour_24 > 12 {
        hour_24 - 12
    } else {
        hour_24
    };
    (hour_12, is_pm)
}

/// Convert 12H hour to 24H format
pub fn hour_12_to_24(hour_12: u8, is_pm: bool) -> u8 {
    if hour_12 == 12 {
        if is_pm {
            12
        } else {
            0
        }
    } else if is_pm {
        hour_12 + 12
    } else {
        hour_12
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24_to_12_conversion() {
        assert_eq!(hour_24_to_12(0), (12, false)); // 12 AM
        assert_eq!(hour_24_to_12(1), (1, false)); // 1 AM
        assert_eq!(hour_24_to_12(11), (11, false)); // 11 AM
        assert_eq!(hour_24_to_12(12), (12, true)); // 12 PM
        assert_eq!(hour_24_to_12(13), (1, true)); // 1 PM
        assert_eq!(hour_24_to_12(23), (11, true)); // 11 PM
    }

    #[test]
    fn test_12_to_24_conversion() {
        assert_eq!(hour_12_to_24(12, false), 0); // 12 AM -> 0
        assert_eq!(hour_12_to_24(1, false), 1); // 1 AM -> 1
        assert_eq!(hour_12_to_24(11, false), 11); // 11 AM -> 11
        assert_eq!(hour_12_to_24(12, true), 12); // 12 PM -> 12
        assert_eq!(hour_12_to_24(1, true), 13); // 1 PM -> 13
        assert_eq!(hour_12_to_24(11, true), 23); // 11 PM -> 23
    }
}
