use warheads::api;

pub struct Client {}

impl Client {
    pub fn launch<W: api::Api>(president_password: &str) -> Result<api::LaunchResponse, String> {
        W::launch(president_password)
    }
}

#[cfg(test)]
pub mod test {
    use user_password_provider::UserPasswordProvider;
    use user_password_provider::test::MockUserPasswordProvider;
    use api_password_generator;
    use warheads::api;
    use warheads::api::Api;
    use warheads::client::Client;

    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct MockApi {}

    impl Api for MockApi {
        fn launch(password: &str) -> Result<api::LaunchResponse, String> {
            let user_password = MockUserPasswordProvider::get_password();
            let response = if password == api_password_generator::build_password(&user_password) {
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
        let launch_response = Client::launch::<MockApi>("bad_password").unwrap();

        assert_eq!(launch_response.result, api::ResultState::Failure);
        assert_eq!(launch_response.message, "Invalid Launch Code");
    }

    #[test]
    fn test_launch_with_good_password() {
        let user_password = MockUserPasswordProvider::get_password();
        let launch_response =
            Client::launch::<MockApi>(&api_password_generator::build_password(&user_password))
                .unwrap();

        assert_eq!(launch_response.result, api::ResultState::Success);
        assert_ne!(launch_response.message, "Invalid Launch Code");
    }
}
