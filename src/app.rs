use dropbox_sdk::{
  default_client::UserAuthDefaultClient,
  oauth2::{get_auth_from_env_or_prompt, Authorization},
};
use std::env::{var, VarError};

fn read_access_token_from_env() -> Result<String, VarError> {
  var("DBX_ACCESS_TOKEN")
}

pub struct App {
  pub client: UserAuthDefaultClient,
}

impl App {
  pub fn new() -> Self {
    let auth = match read_access_token_from_env() {
      Ok(access_token) => Authorization::from_access_token(access_token),
      Err(_) => get_auth_from_env_or_prompt(),
    };
    let client = UserAuthDefaultClient::new(auth);
    App { client }
  }
}
