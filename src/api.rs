extern crate reqwest;
extern crate serde;

extern crate serde_json;

use api_password_generator;
use warheads;

#[derive(Deserialize)]
pub struct LaunchResponse {
    #[serde(rename = "Result")]
    pub result: ApiResultState,
    #[serde(rename = "Message")]
    pub message: String,
}

#[derive(Debug,PartialEq,Deserialize)]
pub enum ApiResultState {
    Success,
    Failure,
}

pub fn launch<W: warheads::Warheads>(president_password: &str) -> Result<LaunchResponse, String> {
    W::launch(president_password)
    // let client = reqwest::Client::new().unwrap();

    // let launch_response: LaunchResponse = client.post(&("http://gitland.azurewebsites.net:80/api/warheads/launch?launchCode="
    //                 .to_owned() + president_password))
    //     .send()?
    //     .json()?;

    // Ok(launch_response)
}

#[test]
fn test_launch_with_bad_password() {
    let launch_response = launch::<warheads::WarheadsMock>("bad_password").unwrap();

    assert_eq!(launch_response.result, ApiResultState::Failure);
    assert_eq!(launch_response.message, "Invalid Launch Code");
}

#[test]
fn test_launch_with_good_password() {
    let launch_response =
        launch::<warheads::WarheadsMock>(&api_password_generator::build_password(api_password_generator::USER_PASSWORD))
            .unwrap();

    assert_eq!(launch_response.result, ApiResultState::Success);
    assert_ne!(launch_response.message, "Invalid Launch Code");
}
