use super::base::DataSource;
use super::mysql::*;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use tracing::log::info;

#[derive(Default, Debug)]
pub struct DataSourceBuilder {
    datasource_type: String,
    config: HashMap<String, String>,
}

impl DataSourceBuilder {
    pub fn new(datasource_type: &str) -> Self {
        DataSourceBuilder {
            datasource_type: datasource_type.to_string(),
            config: HashMap::new(),
        }
    }

    pub fn load_from_env(mut self) -> Result<impl DataSource, Box<dyn Error>> {
        for (key, value) in env::vars() {
            self.config.insert(key, value);
        }
        self.load()
    }

    pub fn load(self) -> Result<impl DataSource, Box<dyn Error>> {
        if self.datasource_type.to_lowercase() == "mysql" {
            info!(
                "processing request to load new {} datasource...",
                &self.datasource_type
            );
            Ok(MySQLDataSource::new(self)?)
        } else {
            Err(Box::<dyn Error>::from("unsupported data source type!!!"))
        }
    }

    pub fn set_config(mut self, key: &str, value: &str) -> Self {
        self.config.insert(key.to_string(), value.to_string());
        self
    }

    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
}
