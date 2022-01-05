use std::env;
use thanks_async_lambda::secrets::get_secrets;

#[tokio::main]
async fn main(
) -> Result<(), aws_sdk_secretsmanager::SdkError<aws_sdk_secretsmanager::error::GetSecretValueError>>
{
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    print!("reading event: {:?}", filename);
    let secrets = get_secrets().await;

    match secrets {
        Ok(_result) => return Ok(()),
        Err(e) => panic!("{:?}", e),
    }
}
