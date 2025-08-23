use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct KafkaConfig {
    pub brokers: String,
    pub topic: String,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub kafka: KafkaConfig,
    pub server: ServerConfig,
}

impl Config {
    /// Загружает конфиг. Сначала проверяет переменную окружения CONFIG_PATH,
    /// потом ищет config.toml рядом с Cargo.toml (dev) или рядом с бинарником (prod).
    pub fn load() -> Self {
        // 1. Переменная окружения
        if let Ok(path) = std::env::var("CONFIG_PATH") {
            return Self::from_file(path);
        }

        // 2. Dev: рядом с Cargo.toml
        let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.toml");
        if dev_path.exists() {
            return Self::from_file(dev_path);
        }

        // 3. Prod: рядом с бинарником
        let exe_path = std::env::current_exe()
            .expect("Failed to get current exe path")
            .parent()
            .expect("Exe has no parent directory")
            .join("config.toml");

        if exe_path.exists() {
            return Self::from_file(exe_path);
        }

        panic!("config.toml not found! Set CONFIG_PATH or put file next to Cargo.toml / binary");
    }

    fn from_file<P: Into<PathBuf>>(path: P) -> Self {
        let path: PathBuf = path.into();
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Failed to read config file: {:?}", path));
        toml::from_str(&content)
            .unwrap_or_else(|_| panic!("Failed to parse config file: {:?}", path))
    }
}
