extern crate chrono;
use self::chrono::prelude::*;
use self::chrono::TimeZone;

const USER_PASSWORD: &str = "USER_PASSWORD";

pub fn build_password(user_password: &str) -> String {
    build_password_with_date(Utc::today(), user_password)
}

fn build_password_with_date(date: Date<Utc>, user_password: &str) -> String {
    let date_string = date.format("%y%m%d").to_string();

    date_string + "-" + user_password
}

#[test]
fn test_build_with_date() {
    let date_1 = chrono::Utc.ymd(2014, 7, 8);

    let password_1 = build_password_with_date(date_1, &USER_PASSWORD);

    assert_eq!(password_1, "140708-".to_string() + &USER_PASSWORD);
}

#[test]
fn test_build() {
    let api_password = build_password(&USER_PASSWORD);
    let expected_password = Utc::today().format("%y%m%d").to_string() + "-" + USER_PASSWORD;

    assert_eq!(api_password, expected_password)
}

#[test]
fn test_build_wrong_password() {
    let api_password = build_password("WRONG_PASSOWRD");
    let expected_password = Utc::today().format("%y%m%d").to_string() + "-" + USER_PASSWORD;

    assert_ne!(api_password, expected_password)
}
