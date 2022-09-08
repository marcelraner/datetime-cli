pub fn leap_years(year: u64) -> u64 {
    (year / 4) - (year / 100) + (year / 400)
}

pub fn is_leap_year(year: u64) -> bool {
    (year % 400) == 0 || ((year % 100) != 0 && (year % 4) == 0)
}

#[cfg(test)]
mod tests {
    use super::{is_leap_year, leap_years};

    #[test]
    fn leap_years_should_return_0_for_year_0() {
        let input_year = 0;
        let result = leap_years(input_year);
        assert_eq!(result, 0);
    }

    #[test]
    fn is_leap_year_should_return_true_when_4true_100true_400true() {
        let input_year = 2000;
        let result = is_leap_year(input_year);
        assert_eq!((input_year % 4) == 0, true);
        assert_eq!((input_year % 100) == 0, true);
        assert_eq!((input_year % 400) == 0, true);
        assert_eq!(result, true);
    }

    #[test]
    fn is_leap_year_should_return_false_when_4true_100true_400false() {
        let input_year = 1700;
        let result = is_leap_year(input_year);
        assert_eq!((input_year % 4) == 0, true);
        assert_eq!((input_year % 100) == 0, true);
        assert_eq!((input_year % 400) == 0, false);
        assert_eq!(result, false);
    }

    #[test]
    fn is_leap_year_should_return_true_when_4true_100false_400false() {
        let input_year = 1604;
        let result = is_leap_year(input_year);
        assert_eq!((input_year % 4) == 0, true);
        assert_eq!((input_year % 100) == 0, false);
        assert_eq!((input_year % 400) == 0, false);
        assert_eq!(result, true);
    }

    #[test]
    fn is_leap_year_should_return_false_when_4false_100false_400false() {
        let input_year = 1;
        let result = is_leap_year(input_year);
        assert_eq!((input_year % 4) == 0, false);
        assert_eq!((input_year % 100) == 0, false);
        assert_eq!((input_year % 400) == 0, false);
        assert_eq!(result, false);
    }
}
