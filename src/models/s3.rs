use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct S3Config {
    #[serde(alias = "S3_BUCKET_NAME")]
    pub bucket_name: String,
    #[serde(alias = "S3_ENDPOINT")]
    pub endpoint: String,
    #[serde(alias = "S3_REGION")]
    pub region: String,
    #[serde(alias = "S3_ACCESS_KEY")]
    pub access_key: String,
    #[serde(alias = "S3_SECRET_KEY")]
    pub secret_key: String,
}
