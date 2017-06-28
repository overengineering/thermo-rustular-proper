extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod api_password_generator;

#[derive(Deserialize)]
struct LaunchResponse {
    #[serde(rename = "Result")]
    result: ApiResultState,
    #[serde(rename = "Message")]
    message: String,
}

#[derive(Debug,PartialEq,Deserialize)]
enum ApiResultState {
    Success,
    Failure,
}

fn main() {
    println!("Hello, world!");
}

fn launch(president_password: &str) -> Result<LaunchResponse, reqwest::Error> {
    let client = reqwest::Client::new().unwrap();

    let launch_response: LaunchResponse = client.post(&("http://gitland.azurewebsites.net:80/api/warheads/launch?launchCode="
                    .to_owned() + president_password))
        .send()?
        .json()?;

    Ok(launch_response)
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