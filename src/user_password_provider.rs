pub trait UserPasswordProvider {
    fn get_password() -> String;
}

#[cfg(test)]
pub mod test {
    pub struct MockUserPasswordProvider {}

    impl ::user_password_provider::UserPasswordProvider for MockUserPasswordProvider {
        fn get_password() -> String {
            "NICEGAMEOFCHESS".to_string()
        }
    }
}