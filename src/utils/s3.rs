use s3::{creds::Credentials, Bucket, Region};

use crate::{models::error::ApiError, S3Config};

// build s3 creds based on the config
pub fn build_creds(conf: &S3Config) -> Result<Credentials, ApiError> {
    let creds = Credentials::new(
        // access key
        Some(&conf.access_key),
        // secret key
        Some(&conf.secret_key),
        // security token
        None,
        // session token
        None,
        // profile
        None,
    );

    match creds {
        // should we return INTERNAL_SERVER_ERROR or UNAUTHORIZED here?
        // wrong creds aren't the users fault after all
        Err(err) => Err(ApiError::Unauthorized),
        // do nothing on correct creds
        Ok(x) => Ok(x),
    }
}

pub fn build_bucket(conf: &S3Config) -> Result<Bucket, ApiError> {
    let bucket_name = conf.bucket_name.as_str();
    let region = Region::Custom {
        region: (&*conf.region).into(),
        endpoint: (&*conf.endpoint).into(),
    };

    let creds = build_creds(conf)?;

    let bucket = Bucket::new(bucket_name, region, creds)?.with_path_style();
    Ok(bucket)
}
