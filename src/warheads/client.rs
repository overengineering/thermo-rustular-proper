extern crate reqwest;
extern crate serde;

extern crate serde_json;

use api_password_generator;

use warheads::api;

pub struct Client {}

impl Client {
    pub fn launch<W: api::Api>(president_password: &str) -> Result<api::LaunchResponse, String> {
        W::launch(president_password)
        // let client = reqwest::Client::new().unwrap();

        // let launch_response: LaunchResponse = client.post(&("http://gitland.azurewebsites.net:80/api/warheads/launch?launchCode="
        //                 .to_owned() + president_password))
        //     .send()?
        //     .json()?;

        // Ok(launch_response)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct MockApi {}

impl api::Api for MockApi {
    fn launch(password: &str) -> Result<api::LaunchResponse, String> {
        let result_state = if
            (password ==
             api_password_generator::build_password(api_password_generator::USER_PASSWORD)) {
            return Ok(api::LaunchResponse {
                          result: api::ResultState::Success,
                          message: "Success".to_string(),
                      });
        } else {
            return Ok(api::LaunchResponse {
                          result: api::ResultState::Failure,
                          message: "Invalid Launch Code".to_string(),
                      });

        };
    }
}

#[test]
fn test_launch_with_bad_password() {
    let launch_response = Client::launch::<MockApi>("bad_password").unwrap();

    assert_eq!(launch_response.result, api::ResultState::Failure);
    assert_eq!(launch_response.message, "Invalid Launch Code");
}

#[test]
fn test_launch_with_good_password() {
    let launch_response = Client::launch::<MockApi>(&api_password_generator::build_password(api_password_generator::USER_PASSWORD))
            .unwrap();

    assert_eq!(launch_response.result, api::ResultState::Success);
    assert_ne!(launch_response.message, "Invalid Launch Code");
}
