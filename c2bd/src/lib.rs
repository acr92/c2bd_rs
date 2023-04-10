use chrono::{DateTime, Datelike, Days, FixedOffset, Weekday};
use std::ops::Add;

pub fn calculate_business_days_between<Tz>(start_date: DateTime<Tz>, end_date: DateTime<Tz>) -> i32
where
    Tz: chrono::TimeZone,
{
    let mut business_days = 0;
    let mut current_date = start_date;
    while current_date <= end_date {
        if current_date.weekday() != Weekday::Sat && current_date.weekday() != Weekday::Sun {
            business_days += 1;
        }
        current_date = current_date.checked_add_days(Days::new(1)).unwrap();
    }

    business_days
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! business_day_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (lhs, rhs, expected) = $value;
            assert_eq!(expected, calculate_business_days_between(
                DateTime::parse_from_rfc3339(lhs).unwrap(),
                DateTime::parse_from_rfc3339(rhs).unwrap())
            );
        }
    )*
    }
}

    business_day_tests! {
        gets_1_business_days_between_2021_01_01_and_2021_01_01: ("2021-01-01T00:00:00+00:00", "2021-01-01T00:00:00+00:00", 1),
        gets_1_business_day_between_2021_01_01_and_2021_01_02: ("2021-01-01T00:00:00+00:00", "2021-01-02T00:00:00+00:00", 1),
        gets_1_business_day_between_2021_01_01_and_2021_01_03: ("2021-01-01T00:00:00+00:00", "2021-01-03T00:00:00+00:00", 1),
        gets_2_business_days_between_2021_01_01_and_2021_01_04: ("2021-01-01T00:00:00+00:00", "2021-01-04T00:00:00+00:00", 2),
        gets_3_business_days_between_2021_01_01_and_2021_01_05: ("2021-01-01T00:00:00+00:00", "2021-01-05T00:00:00+00:00", 3),
        gets_4_business_days_between_2021_01_01_and_2021_01_06: ("2021-01-01T00:00:00+00:00", "2021-01-06T00:00:00+00:00", 4),
        gets_5_business_days_between_2021_01_01_and_2021_01_07: ("2021-01-01T00:00:00+00:00", "2021-01-07T00:00:00+00:00", 5),
        gets_6_business_days_between_2021_01_01_and_2021_01_08: ("2021-01-01T00:00:00+00:00", "2021-01-08T00:00:00+00:00", 6),
        gets_23_business_days_between_2023_03_01_and_2023_03_31: ("2023-03-01T00:00:00+00:00", "2023-03-31T00:00:00+00:00", 23),
    }
}
