use config::{Config, ConfigError};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub database_url: String,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("configuration"))
        .build()
        .unwrap();

    settings.try_deserialize()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("{}", self.database_url)
    }

    // pub fn connection_string_without_db(&self) -> String {
    //     format!(
    //         "postgres://{}:{}@{}:{}",
    //         self.username, self.password, self.host, self.port,
    //     )
    // }
}
