mod structs;
mod export;
mod cli;
mod serialization;

fn main() {
    let opt = cli::Opts::load();
    match opt.cmd() {
        cli::SubCommand::Export(opts) => {
            let (dir, file_type) = opts.unroll();
            export::export(dir.as_str(), file_type);
        }
    }
}
