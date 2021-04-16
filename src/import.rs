use crate::serialization::FileType;
use std::path::PathBuf;
use std::str::FromStr;
use super::structs::Export;
use std::process::Command;

pub fn import(file: &str, filetype: Option<FileType>) {
    let path = PathBuf::from_str(file).unwrap();
    let filetype = filetype.or(
        path.extension()
            .map(|s| s.to_os_string().into_string().unwrap())
            .and_then(|s| FileType::from_str(s.as_str())))
            .expect("Unknown filetype!");

    let s = std::fs::read_to_string(path).unwrap();

    let loaded: Export = match filetype {
        FileType::Json => serde_json::from_str(s.as_str()).unwrap(),
        FileType::Toml => toml::from_str(s.as_str()).unwrap(),
        FileType::Yaml => serde_yaml::from_str(s.as_str()).unwrap(),
    };

    println!("Adding buckets...");

    let add_bucket = |name: &str| {
        Command::new("cmd").args(&["/C", format!("scoop bucket add {}", name).as_str()]).output().unwrap()
    };

    for bucket in loaded.buckets() {
        println!("Adding bucket [{}]", bucket);
        let output = add_bucket(bucket);
        if output.status.success() {
            let out = String::from_utf8(output.stdout).unwrap();
            println!("{}", out);
        } else {
            println!("Error adding bucket {}", bucket);
            let err = String::from_utf8(output.stderr).unwrap();
            println!("STDERR:\n{}", err);
        }
    }
}