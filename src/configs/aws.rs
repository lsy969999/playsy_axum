use aws_config::BehaviorVersion;

use crate::utils;

pub async fn init_s3_client() -> aws_sdk_s3::Client {
    let settings = utils::config::get_config_aws();
    std::env::set_var("AWS_ACCESS_KEY_ID", settings.aws_access_key_id.clone());
    std::env::set_var("AWS_SECRET_ACCESS_KEY", settings.aws_secret_access_key.clone());
    std::env::set_var("AWS_REGION", settings.aws_region.clone());
    std::env::set_var("AWS_S3_BUCKET", settings.aws_s3_bucket.clone());
    std::env::set_var("BUCKET_URL", settings.aws_s3_bucket_url.clone());

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&config);
    client
}