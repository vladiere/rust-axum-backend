use std::sync::OnceLock;

use crate::{
    envs::{get_env, get_env_parse},
    error,
};

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - while loading - error: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    pub SERVER_URL: String,
    pub SERVER_PORT: u32,

    pub DB_NAME: String,
    pub DB_HOST: String,
    pub DB_USER: String,
    pub DB_PASS: String,
    pub DB_PORT: u32,
}

impl CoreConfig {
    fn load_from_env() -> error::Result<CoreConfig> {
        Ok(CoreConfig {
            SERVER_URL: get_env("SERVER_URL")?,
            SERVER_PORT: get_env_parse("SERVER_PORT")?,

            DB_NAME: get_env("DB_NAME")?,
            DB_HOST: get_env("DB_HOST")?,
            DB_USER: get_env("DB_USER")?,
            DB_PASS: get_env("DB_PASS")?,
            DB_PORT: get_env_parse("DB_PORT")?,
        })
    }
}
