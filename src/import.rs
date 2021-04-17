use crate::serialization::FileType;
use std::path::PathBuf;
use std::str::FromStr;
use super::structs::Export;
use std::process::Command;
use rayon::prelude::*;
use crate::structs::ExportEntry;

pub fn import(file: &str, filetype: Option<FileType>, allow_any_version: bool) {
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
        println!("Adding bucket [{}]", name);
        let output = Command::new("cmd")
            .args(&["/C", format!("scoop bucket add {}", name).as_str()])
            .output()
            .unwrap();
        if output.status.success() {
            let out = String::from_utf8(output.stdout).unwrap();
            println!("{}", out);
        } else {
            println!("Error adding bucket {}", name);
            let err = String::from_utf8(output.stderr).unwrap();
            println!("STDERR:\n{}", err);
        }
    };

    loaded.buckets_vec().par_iter().for_each(|s| {
        add_bucket(s)
    });

    let download = |entry: &ExportEntry| {
        let cmd = if !allow_any_version && entry.version().is_some() {
            format!("scoop install {}@{}", entry.name(), entry.version().unwrap())
        } else {
            format!("scoop install {}", entry.name())
        };
        println!("Downloading with cmd {}", cmd);
        let output = Command::new("cmd")
            .args(&["/C", cmd.as_str()])
            .output()
            .unwrap();

        if output.status.success() {
            println!("Downloaded {} {} successfully!", entry.name(), match entry.version() {
                Some(v) => format!("version {}", v),
                None => "".to_owned(),
            });
        } else {
            let err = String::from_utf8(output.stderr).unwrap();
            let out = String::from_utf8(output.stdout).unwrap();
            println!("Failed to download {} {}!", entry.name(), match entry.version() {
                Some(v) => format!("version {}", v),
                None => "".to_owned(),
            });
            if !out.is_empty() {
                println!("{}", out);
            }
            if !err.is_empty() {
                println!("{}", err);
            }
        }
    };

    loaded.entries().par_iter().for_each(|e| {
        download(e)
    })
}