use anyhow::anyhow;
use chrono::{DateTime, Utc};
use std::{collections::HashMap, env, fs::File, io::Read, path::Path};
use toml::Value;

//use crate::traits::Repository;

#[derive(Clone, Debug)]
pub enum Configuration {
    Map(HashMap<String, Configuration>),
    Array(Vec<Configuration>),
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    DateTime(DateTime<Utc>),
}

impl Configuration {
    pub fn merge(&self, setting_value: &Configuration) -> Configuration {
        match (self, setting_value) {
            (Configuration::Map(old_map), Configuration::Map(new_map)) => {
                let mut hashmap = HashMap::new();
                for (k, v) in old_map {
                    if new_map.contains_key(k) {
                        hashmap.insert(k.clone(), v.merge(new_map.get(k).unwrap()));
                    } else {
                        hashmap.insert(k.clone(), v.clone());
                    }
                }
                for (k, v) in new_map {
                    if !old_map.contains_key(k) {
                        hashmap.insert(k.clone(), v.clone());
                    }
                }
                Configuration::Map(hashmap)
            }
            _ => setting_value.clone(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Configuration> {
        match self {
            Configuration::Map(hashmap) => hashmap.get(key),
            _ => None,
        }
    }
}

impl From<Value> for Configuration {
    fn from(value: Value) -> Self {
        if value.is_table() {
            let mut hashmap = HashMap::new();
            for (k, v) in value.as_table().unwrap() {
                hashmap.insert(k.clone(), Self::from(v.clone()));
            }
            Self::Map(hashmap)
        } else if value.is_array() {
            Self::Array(
                value
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| Self::from(v.clone()))
                    .collect(),
            )
        } else if value.is_bool() {
            Self::Bool(value.as_bool().unwrap())
        } else if value.is_str() {
            Self::String(value.as_str().unwrap().to_string())
        } else if value.is_integer() {
            Self::Int(value.as_integer().unwrap())
        } else if value.is_float() {
            Self::Float(value.as_float().unwrap())
        } else if value.is_datetime() {
            let dt_str = value.as_datetime().unwrap().to_string();
            let fixed_dt = DateTime::parse_from_rfc3339(&dt_str).unwrap();
            Self::DateTime(fixed_dt.to_utc())
        } else {
            panic!("Unhandle toml type")
        }
    }
}

pub(crate) fn load_configuration() -> Configuration {
    let current_exe_dir = env::current_exe().unwrap();
    let parent_dir = current_exe_dir.parent().unwrap();
    let mut config_path = parent_dir.join("config.toml");
    let mut secret_path = parent_dir.join("secret.toml");
    let pwd = env::var("PWD").unwrap();
    let current_dir = Path::new(&pwd);

    eprintln!("config path: {:?}", config_path);
    eprintln!("secret path: {:?}", secret_path);
    if !config_path.exists() {
        config_path = current_dir.join("config.toml");
    }
    if !secret_path.exists() {
        secret_path = current_dir.join("secret.toml");
    }
    eprintln!("config path: {:?}", config_path);
    eprintln!("secret path: {:?}", secret_path);
    let mut config = File::open(&config_path)
        .map_err(|e| anyhow!("failed to load config.toml: {:?}", e))
        .unwrap();
    let mut secret = File::open(&secret_path)
        .map_err(|e| anyhow!("failed to load secret.toml: {:?}", e))
        .unwrap();

    let config_content = &mut String::new();
    let secret_content = &mut String::new();

    config.read_to_string(config_content).unwrap();
    secret.read_to_string(secret_content).unwrap();

    let parsed_config: Value = toml::from_str(config_content).unwrap();
    let parsed_secret: Value = toml::from_str(secret_content).unwrap();

    Configuration::from(parsed_config.clone()).merge(&Configuration::from(parsed_secret.clone()))
}

// impl Repository<String> for Configuration {
//     fn delete(&self, repository_connexion: &String) {

//     }

//     fn save(&self, repository_connexion: &String) {

//     }
// }
