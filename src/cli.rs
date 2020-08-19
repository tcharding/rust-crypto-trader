use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub struct Options {
    /// Path to configuration file
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    pub config_file: Option<PathBuf>,

    /// Dump the current configuration and exit
    #[structopt(long = "dump-config")]
    pub dump_config: bool,

    #[structopt(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Clone, Copy, Debug, StructOpt)]
pub enum Cmd {
    Test,
    SpreadBot,
}
