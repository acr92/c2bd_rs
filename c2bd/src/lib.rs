use chrono::{DateTime, Datelike, Days, FixedOffset, NaiveDate, Weekday};

pub struct BusinessDays {
    start_date: DateTime<FixedOffset>,
    end_date: DateTime<FixedOffset>,
    excluded_days: Vec<NaiveDate>,
}

impl BusinessDays {
    pub fn new(start_date: DateTime<FixedOffset>, end_date: DateTime<FixedOffset>) -> Self {
        Self {
            start_date,
            end_date,
            excluded_days: vec![],
        }
    }

    pub fn exclude_day(&mut self, day: DateTime<FixedOffset>) {
        self.excluded_days.push(day.date_naive());
    }

    pub fn calculate(&self) -> i32 {
        let mut business_days = 0;
        let mut current_date = self.start_date;
        while current_date <= self.end_date {
            business_days += match current_date.weekday() {
                Weekday::Sat | Weekday::Sun => 0,
                _ if self.excluded_days.contains(&current_date.date_naive()) => 0,
                _ => 1,
            };

            current_date = current_date.checked_add_days(Days::new(1)).unwrap();
        }

        business_days
    }
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

            let business_days = BusinessDays::new(
                DateTime::parse_from_rfc3339(lhs).unwrap(),
                DateTime::parse_from_rfc3339(rhs).unwrap(),
            );

            assert_eq!(expected, business_days.calculate());
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

    #[test]
    fn gets_17_business_days_between_2023_04_01_and_2023_04_30_when_excluding_easter() {
        let mut business_days = BusinessDays::new(
            DateTime::parse_from_rfc3339("2023-04-01T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-04-30T00:00:00+00:00").unwrap(),
        );

        business_days
            .exclude_day(DateTime::parse_from_rfc3339("2023-04-06T00:00:00+00:00").unwrap());
        business_days
            .exclude_day(DateTime::parse_from_rfc3339("2023-04-07T00:00:00+00:00").unwrap());
        business_days
            .exclude_day(DateTime::parse_from_rfc3339("2023-04-10T00:00:00+00:00").unwrap());

        assert_eq!(17, business_days.calculate());
    }

    #[test]
    fn gets_17_business_days_between_2023_04_01_and_2023_04_30_when_excluding_easter_with_tz() {
        let mut business_days = BusinessDays::new(
            DateTime::parse_from_rfc3339("2023-04-01T00:00:00+00:00").unwrap(),
            DateTime::parse_from_rfc3339("2023-04-30T00:00:00+02:00").unwrap(),
        );

        business_days
            .exclude_day(DateTime::parse_from_rfc3339("2023-04-06T00:00:00+01:00").unwrap());
        business_days
            .exclude_day(DateTime::parse_from_rfc3339("2023-04-07T10:00:00+00:00").unwrap());
        business_days
            .exclude_day(DateTime::parse_from_rfc3339("2023-04-10T00:00:00+00:00").unwrap());

        assert_eq!(17, business_days.calculate());
    }
}
