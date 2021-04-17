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
    Export(ExportOpts),
    Import(ImportOpts),
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

#[derive(Clap, Debug, Clone)]
pub struct ImportOpts {
    location: String,
    #[clap(short, long, arg_enum)]
    filetype: Option<FileType>,
    #[clap(long)]
    exact_version: bool,
}

impl ImportOpts {
    pub fn location(&self) -> &str {
        &self.location
    }

    pub fn filetype(&self) -> Option<FileType> {
        self.filetype.clone()
    }

    pub fn allow_any_version(&self) -> bool {
        !self.exact_version
    }
}
