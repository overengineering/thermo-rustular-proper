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

fn launch(password: String) -> Result<LaunchResponse, String> {
    Ok(LaunchResponse {
           result: ApiResultState::Failure,
           message: "".to_string(),
       })
}

#[test]
fn test_launch_with_bad_password() {
    let launch_response = launch("password".to_string());

    assert_eq!(launch_response.unwrap().result, ApiResultState::Failure);
}

#[test]
fn test_launch_with_good_password() {
    let launch_response = launch("NICEGAMEOFCHESS".to_string());

    assert_eq!(launch_response.unwrap().result, ApiResultState::Success);
}