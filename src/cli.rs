use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap)]
pub enum Command {
    Cpu,
}

pub fn parse() -> Opts {
    Opts::parse()
}
