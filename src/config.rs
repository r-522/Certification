use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub email_hmac_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            email_hmac_secret: env::var("EMAIL_HMAC_SECRET")?,
        })
    }
}
