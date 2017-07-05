use api;
use api_password_generator;

pub trait Warheads {
    //     fn status() -> Option<bool>;
    fn launch(password: &str) -> Result<api::LaunchResponse, String>;
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct WarheadsMock {}

impl Warheads for WarheadsMock {
    fn launch(password: &str) -> Result<api::LaunchResponse, String> {
        let result_state = if
            (password ==
             api_password_generator::build_password(api_password_generator::USER_PASSWORD)) {
            return Ok(api::LaunchResponse {
                          result: api::ApiResultState::Success,
                          message: "Success".to_string(),
                      });
        } else {
            return Ok(api::LaunchResponse {
                          result: api::ApiResultState::Failure,
                          message: "Invalid Launch Code".to_string(),
                      });

        };
    }
}
