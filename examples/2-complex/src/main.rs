use clap::{Parser, ArgAction};
use blkrs::run_lsblk;

#[derive(Parser)]
#[command(
    version = "0.0.1",
    author = "Alfredo Deza",
    about = "lsblk in Rust"
)]
struct Opts {

    #[clap(short, long, help = "Provide better output for debugging purposes", default_value_t = false)]
    debug: bool,

    #[clap(short, long, action = ArgAction::Count)]
    verbose_level: u8,

    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    #[clap(name="info", about = "get info about a device")]
    Info(InfoOpts),
}

#[derive(Parser)]
struct InfoOpts {
    #[clap(help = "device to get info about")]
    device: String,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts.debug);
    match opts.verbose_level {
        0 => println!("no verbosity"),
        1 => println!("some verbosity"),
        2 => println!("more verbosity"),
        _ => println!("too much verbosity"),
    }
    match opts.cmd {
        Command::Info(info) => {
            let device = info.device;
            let output = run_lsblk(&device);
            println!("{}", output);
        }
    }
}