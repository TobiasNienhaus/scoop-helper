mod structs;
mod export;
mod cli;
mod serialization;
mod import;

fn main() {
    let opt = cli::Opts::load();
    match opt.cmd() {
        cli::SubCommand::Export(opts) => {
            let (dir, file_type) = opts.unroll();
            export::export(dir.as_str(), file_type);
        }
        cli::SubCommand::Import(opts) => {
            import::import(opts.location(), opts.filetype(), opts.allow_any_version())
        }
    }
}
