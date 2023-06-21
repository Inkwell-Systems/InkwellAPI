use config::FileFormat;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;

#[derive(Deserialize)]
pub struct Settings {
    pub app_settings: ApplicationSettings,
    pub db_settings: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub address: String,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub username: String,
    pub password: String,
    pub db_name: Secret<String>,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn get_connection_details(&self) -> PgConnectOptions {
        let mut options = self
            .get_connection_details_no_db()
            .database(self.db_name.expose_secret());
        options.log_statements(log::LevelFilter::Trace);

        options
    }

    pub fn get_connection_details_no_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .ssl_mode(ssl_mode)
    }
}

pub enum AppEnv {
    Local,
    Production,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let base_path =
        std::env::current_dir().expect("Failed to get current directory.");
    let config_dir = base_path.join("config");

    let environment: AppEnv = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Error parsing app environment.");
    let env_filename = format!("{}.json", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::new(
            config_dir
                .join("base.json")
                .to_str()
                .expect("Error reading contents of base.json"),
            FileFormat::Json5,
        ))
        .add_source(config::File::new(
            config_dir
                .join(env_filename.clone())
                .to_str()
                .unwrap_or_else(|| {
                    panic!("Error reading contents of {}.", env_filename)
                }),
            FileFormat::Json5,
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl AppEnv {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppEnv::Local => "local",
            AppEnv::Production => "production",
        }
    }
}

impl TryFrom<String> for AppEnv {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
Use either `local` or `production`.",
                other
            )),
        }
    }
}
