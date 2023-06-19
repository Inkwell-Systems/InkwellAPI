use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub db_settings: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub db_name: String,
}

impl DatabaseSettings {
    pub fn get_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.json",
            config::FileFormat::Json5,
        ))
        .build()
        .unwrap();

    settings.try_deserialize::<Settings>()
}
