use serde::Deserialize;

use std::error;

#[derive(Debug)]
pub struct Secrets {
    pub slack_bot_oauth_token: String,
    pub slack_signing_secret: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct SecretsJson {
    SLACK_BOT_OAUTH_TOKEN: String,
    SLACK_SIGNING_SECRET: String,
}

pub async fn get_secrets() -> Result<Secrets, Box<dyn error::Error>> {
    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_secretsmanager::Client::new(&shared_config);
    let response = client
        .get_secret_value()
        .secret_id("anu-thanks-secrets")
        .send()
        .await;
    let secret_response = response?;
    let secret_string = secret_response.secret_string.ok_or("")?;
    let secrets_json: SecretsJson = serde_json::from_str(&secret_string)?;
    // print!("{:?}", secrets_json);
    let secrets = Secrets {
        slack_bot_oauth_token: secrets_json.SLACK_BOT_OAUTH_TOKEN,
        slack_signing_secret: secrets_json.SLACK_SIGNING_SECRET,
    };
    Ok(secrets)
}
