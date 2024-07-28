use aws_sdk_s3::operation::put_object::PutObjectOutput;
use axum_typed_multipart::FieldData;

pub async fn s3_put_profile_image(
    client: aws_sdk_s3::Client,
    profile_img: FieldData<axum::body::Bytes>,
    key: String,
) -> anyhow::Result<PutObjectOutput> {
    let settings = super::config::get_config_aws();
    let bucket = settings.aws_s3_bucket.clone();
    let resp = client
        .put_object()
        .bucket(&bucket)
        .key(&key)
        .body(profile_img.contents.into())
        .send()
        .await?;
    Ok(resp)
}