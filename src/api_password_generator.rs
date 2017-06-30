extern crate chrono;
use self::chrono::prelude::*;
use self::chrono::TimeZone;

pub fn build_password<'a>() -> &'a str {
    "NICEGAMEOFCHESS"
}

fn build_password_with_date(date: Date<Utc>) -> String {
    let date_string = date.format("%y%m%d").to_string();

    date_string + "NICEGAMEOFCHESS"
}

// #[test]
// fn password_is_correct() {
    
// }

// pub buil 
//     build with date today

// priv buil with date

#[test]
fn test_build_with_date() {
    let date_1 = chrono::Utc.ymd(2014, 7, 8);

    let password_1 = build_password_with_date(date_1);

    assert_eq!(password_1, "140708-NICEGAMEOFCHESS");
}