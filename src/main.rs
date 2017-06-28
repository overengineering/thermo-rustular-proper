mod api_password_generator;

struct LaunchResponse {
    result: ApiResultState,
    message: String,
}

#[derive(Debug,PartialEq)]
enum ApiResultState {
    Success,
    Failure,
}

fn main() {
    println!("Hello, world!");
}

fn launch(password: &str) -> Result<LaunchResponse, String> {
    if password == "NICEGAMEOFCHESS" {
        Ok(LaunchResponse {
               result: ApiResultState::Success,
               message: "".to_string(),
           })
    } else {
        Ok(LaunchResponse {
               result: ApiResultState::Failure,
               message: "".to_string(),
           })
    }
}

#[test]
fn test_launch_with_bad_password() {
    let launch_response = launch("password").unwrap();



    assert_eq!(launch_response.result, ApiResultState::Failure);
    assert_eq!(launch_response.message, "Invalid Launch Code");
}

#[test]
fn test_launch_with_good_password() {
    let launch_response = launch(api_password_generator::build_password()).unwrap();

    assert_eq!(launch_response.result, ApiResultState::Success);
    assert_ne!(launch_response.message, "Invalid Launch Code");
}