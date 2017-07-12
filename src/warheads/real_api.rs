extern crate reqwest;
extern crate serde;

extern crate serde_json;

use warheads::api;
use warheads::api::Api;
use api_password_generator;

pub struct RealApi {}

impl RealApi {
    fn post(client: &mut reqwest::Client, password: &str) -> reqwest::Result<reqwest::Response> {
        let url = "http://gitland.azurewebsites.net:80/api/warheads/launch?launchCode="
            .to_owned() + password;

        client.post(&url).send()
    }
}

impl api::Api for RealApi {
    fn launch(password: &str) -> Result<api::LaunchResponse, String> {
        let mut client = match reqwest::Client::new() {
            Ok(client) => client,
            Err(_) => return Err("error creating client".to_string()),
        };

        let mut request_response: reqwest::Response = match Self::post(&mut client, password) {
            Ok(response) => response,
            Err(_) => return Err("error sending post request".to_string()),
        };

        let launch_response: api::LaunchResponse = match request_response.json() {
            Ok(response) => response,
            Err(_) => return Err("error deserialising post response".to_string()),
        };

        Ok(launch_response)
    }
}

#[test]
fn test_launch_call() {
    assert!(RealApi::launch(&api_password_generator::build_password(api_password_generator::USER_PASSWORD)).is_ok())
}