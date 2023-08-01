use clap::Parser;
use blkrs::run_lsblk;

#[derive(Parser)]
#[command(
    version = "0.0.1",
    author = "Alfredo Deza",
    about = "lsblk in Rust"
)]
struct Opts {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    #[clap(name="part", about = "get info about a device")]
    Info(InfoOpts),
}

#[derive(Parser)]
struct InfoOpts {
    #[clap(help = "device to get info about")]
    device: String,
}

fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        Command::Info(info) => {
            let device = info.device;
            let output = run_lsblk(&device);
            println!("{}", output);
        }
    }
}