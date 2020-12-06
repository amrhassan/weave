mod cli;
mod error;
mod command_cpu;

use error::WeaveResult;

fn main() -> WeaveResult<()> {
    let opts = cli::parse();
    match opts.command {
        cli::Command::Cpu => command_cpu::run(),
    }
}
