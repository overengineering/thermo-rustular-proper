#[derive(Deserialize)]
pub struct LaunchResponse {
    #[serde(rename = "Result")]
    pub result: ResultState,
    #[serde(rename = "Message")]
    pub message: String,
}

#[derive(Debug,PartialEq,Deserialize)]
pub enum ResultState {
    Success,
    Failure,
}

pub trait Api {
    //     fn status() -> Option<bool>;
    fn launch(password: &str) -> Result<LaunchResponse, String>;
}
