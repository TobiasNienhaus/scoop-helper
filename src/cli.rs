use clap::Clap;
use std::env;
use super::serialization::FileType;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    cmd: SubCommand,
}

impl Opts {
    pub fn load() -> Opts {
        Opts::parse()
    }

    pub fn cmd(&self) -> SubCommand {
        self.cmd.clone()
    }
}

#[derive(Clap, Debug, Clone)]
pub enum SubCommand {
    Export(ExportOpts)
}

#[derive(Clap, Debug, Clone)]
pub struct ExportOpts {
    location: Option<String>,
    #[clap(short, long, arg_enum, default_value="json")]
    filetype: FileType,
}

impl ExportOpts {
    pub fn unroll(self) -> (String, FileType) {
        (
            self.location.unwrap_or(env::current_dir().unwrap().into_os_string().into_string().unwrap()),
            self.filetype,
        )
    }
}

impl FileType {
    pub fn extension(&self) -> &'static str {
        match self {
            FileType::Json => "json",
            FileType::Toml => "toml",
            FileType::Yaml => "yaml",
        }
    }
}
