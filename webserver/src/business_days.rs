use actix_web::{get, web, HttpResponse, Responder};
use c2bd::BusinessDays;
use chrono::{DateTime, FixedOffset, Utc};

#[get("/")]
pub async fn calculate_business_days(
    target_date: web::Data<DateTime<FixedOffset>>,
) -> impl Responder {
    let template = include_str!("../templates/index.html");

    let current_datetime = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
    let target_date = *target_date.get_ref();

    let business_days = BusinessDays::new(current_datetime, target_date);
    let mut business_days_exclude_vacation = BusinessDays::new(current_datetime, target_date);
    populate_vacation_days(&mut business_days_exclude_vacation);

    let result = template
        .replace(
            "{{business_days_left}}",
            business_days.calculate().to_string().as_str(),
        )
        .replace(
            "{{days_left}}",
            business_days_exclude_vacation
                .calculate()
                .to_string()
                .as_str(),
        );

    HttpResponse::Ok().content_type("text/html").body(result)
}

fn populate_vacation_days(business_days_exclude_vacation: &mut BusinessDays) {
    // Easter
    business_days_exclude_vacation
        .exclude_day(DateTime::parse_from_rfc3339("2023-04-10T00:00:00+00:00").unwrap());

    // Company Gathering
    business_days_exclude_vacation
        .exclude_day(DateTime::parse_from_rfc3339("2023-04-26T00:00:00+00:00").unwrap());
    business_days_exclude_vacation
        .exclude_day(DateTime::parse_from_rfc3339("2023-04-27T00:00:00+00:00").unwrap());
    business_days_exclude_vacation
        .exclude_day(DateTime::parse_from_rfc3339("2023-04-28T00:00:00+00:00").unwrap());

    // May
    business_days_exclude_vacation
        .exclude_day(DateTime::parse_from_rfc3339("2023-05-01T00:00:00+00:00").unwrap());
    business_days_exclude_vacation
        .exclude_day(DateTime::parse_from_rfc3339("2023-05-17T00:00:00+00:00").unwrap());
    business_days_exclude_vacation
        .exclude_day(DateTime::parse_from_rfc3339("2023-05-18T00:00:00+00:00").unwrap());
    business_days_exclude_vacation
        .exclude_day(DateTime::parse_from_rfc3339("2023-05-29T00:00:00+00:00").unwrap());
}
