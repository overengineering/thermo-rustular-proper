use warheads::api;
use api_password_generator;

pub struct Client {}

impl Client {
    pub fn launch<W: api::Api>(user_password: &str) -> Result<api::LaunchResponse, String> {
        let api_password = api_password_generator::build_password(&user_password);

        W::launch(&api_password)
    }
}

#[cfg(test)]
pub mod test {
    use warheads::api;
    use warheads::api::Api;
    use warheads::client::Client;
    use user_password_provider::UserPasswordProvider;
    use user_password_provider::test::MockUserPasswordProvider;
    use api_password_generator;

    #[derive(Debug)]
    pub struct ClientMockApi {}

    impl Api for ClientMockApi {
        fn launch(api_password: &str) -> Result<api::LaunchResponse, String> {
            let mock_user_password = MockUserPasswordProvider::get_password();
            let mock_api_password = api_password_generator::build_password(&mock_user_password);

            let response = if api_password == mock_api_password {
                api::LaunchResponse {
                    result: ::warheads::api::ResultState::Success,
                    message: "Success".to_string(),
                }
            } else {
                api::LaunchResponse {
                    result: ::warheads::api::ResultState::Failure,
                    message: "Invalid Launch Code".to_string(),
                }
            };

            Ok(response)
        }
    }

    #[test]
    fn test_launch_with_bad_password() {
        let launch_response = Client::launch::<ClientMockApi>("bad_password").unwrap();

        assert_eq!(launch_response.result, api::ResultState::Failure);
        assert_eq!(launch_response.message, "Invalid Launch Code");
    }

    #[test]
    fn test_launch_with_good_password() {
        let user_password = MockUserPasswordProvider::get_password();
        let launch_response = Client::launch::<ClientMockApi>(&user_password).unwrap();

        assert_eq!(launch_response.result, api::ResultState::Success);
        assert_ne!(launch_response.message, "Invalid Launch Code");
    }
}
