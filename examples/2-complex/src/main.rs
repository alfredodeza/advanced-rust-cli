use clap::{Parser, ArgAction};
use blkrs::run_lsblk;

#[derive(Parser)]
#[command(name = "lsblk", version = "0.0.1", author = "Alfredo Deza", about = "lsblk in Rust")]
struct Opts {
    #[clap(short, long, action = ArgAction::Count)]
    verbose_level: u8,

    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    #[clap(name = "info", about = "Get information about a device")]
    Info(InfoOpts),
}

#[derive(Parser)]
struct InfoOpts {
    #[clap(help = "Device to query")]
    device: String,
}

fn main() {
    let opts = Opts::parse();

    // Example usage of the global flags
    if opts.debug {
        println!("Debug mode enabled");
    }

    match opts.cmd {
        Command::Info(info_opts) => {
            // Example usage of the verbosity level
            match opts.verbose_level {
                0 => {
                    // Quiet mode
                }
                1 => {
                    println!("Running in verbose mode level 1");
                }
                2 => {
                    println!("Running in verbose mode level 2");
                }
                3 | _ => {
                    println!("Running in verbose mode level 3");
                }
            }

            let output = serde_json::to_string(&run_lsblk(&info_opts.device)).unwrap();
            println!("{}", output);
        }
    }
}
