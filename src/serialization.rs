use serde::{Deserialize, Serialize};
use clap::Clap;

#[derive(Clap, Debug, Clone)]
pub enum FileType {
    Json,
    Toml,
    Yaml,
}

pub fn to_string<T: Serialize>(file_type: &FileType, t: &T) -> String {
    match file_type {
        FileType::Json => {
            serde_json::to_string_pretty(t).unwrap()
        }
        FileType::Toml => {
            toml::to_string_pretty(t).unwrap()
        }
        FileType::Yaml => {
            serde_yaml::to_string(t).unwrap()
        }
    }
}

// TODO
// pub fn from_str<'a, T: >(file_type: FileType, s: &str) -> T {
//     match file_type {
//         FileType::Json => {
//             serde_yaml::from_str(s).unwrap()
//         }
//         FileType::Toml => {
//             toml::from_str(s).unwrap()
//         }
//         FileType::Yaml => {
//             serde_yaml::from_str(s).unwrap()
//         }
//     }
// }
