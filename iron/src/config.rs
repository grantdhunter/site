use toml;
use std::fs::File;
use std::result;
use std::io;
use std::io::Read;

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Decode,
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::Io(err)
    }
}

type Result<T> = result::Result<T, ConfigError>;

#[derive(Serialize, Deserialize, RustcDecodable, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub db: DbConfig,
}

#[derive(Serialize, Deserialize, RustcDecodable, Debug)]
pub struct ServerConfig {
    ip: String,
    port: u16,
}

#[derive(Serialize, Deserialize, RustcDecodable, Debug)]
pub struct DbConfig {
    url: String,
    user: String,
    password: String,
}


impl Config {
    pub fn decode(config_path: &str) -> Result<Config> {
        let mut file_str = String::new();
        let mut file = File::open(config_path)?;
        file.read_to_string(&mut file_str)?;
        toml::decode_str(&file_str).ok_or(ConfigError::Decode)
    }

    pub fn ip_and_port(&self) -> String {
        format!("{}:{}", self.server.ip, self.server.port)
    }

    pub fn db_connection(&self) -> String {
        format!("postgres://{}:{}@{}/site",
                self.db.user,
                self.db.password,
                self.db.url)
    }
}


pub fn get() -> Config {
    Config::decode("./config.toml").expect("No config.toml found!")
}
