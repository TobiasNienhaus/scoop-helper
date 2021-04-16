use std::process::Command;
use std::env;
use super::structs::*;
use chrono::Utc;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::str::FromStr;
use std::io::Write;
use lazy_static::lazy_static;
use regex::Regex;
use super::serialization::{FileType, to_string};

lazy_static!{
    static ref EXPORT_PARSER: Regex = Regex::new(r"((?P<name>[\w-]+) \(v:(?P<version>[\w.-]+)\) \[(?P<bucket>\w+)\])").unwrap();
}

pub fn export(dir: &str, file_type: FileType) {
    println!("Saving in {}", dir);
    println!("Executed from: {:?}", env::current_dir().unwrap());
    let output = Command::new("cmd").args(&["/C", "scoop export"]).output().unwrap();
    if output.status.success() {
        let output = String::from_utf8(output.stdout).unwrap();
        let mut export = Export::new();
        for line in output.lines() {
            if let Some(c) = EXPORT_PARSER.captures(line) {
                if let Some(name) = c.name("name") {
                    export.add_entry(name.as_str(),
                                     c.name("version").map(|m| m.as_str()),
                                     c.name("bucket").map(|m| m.as_str()));
                } else {
                    println!("Invalid line!");
                }
            }
        }
        let data = to_string(&file_type, &export);
        let filename = format!("scoop-export_{}.{}",
                               Utc::now().format("%F_%H-%M-%S_%Z"),
                               file_type.extension());
        let full_path = PathBuf::from_str(dir).unwrap().join(filename.as_str());
        println!("Saving to {:?}", full_path);
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .truncate(true)
            .open(&full_path)
            .unwrap();
        file.write_all(data.as_bytes()).unwrap();
    } else {
        println!("Scoop did not execute successfully!");
        if let Some(code) = output.status.code() {
            println!("Exit code: {}", code);
        }
        println!("STDOUT:\n{}", String::from_utf8(output.stdout).unwrap());
        println!("STDERR:\n{}", String::from_utf8(output.stderr).unwrap());
    }
}