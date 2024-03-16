/// Example usage:
/// ```
/// $ monitor ls -- -l
/// ```
/// This will run the `ls -l` command repeatedly until it fails. If the command
/// fails, an error message will be logged. Note that -- is used to separate the
/// command from its arguments.
use chrono::Local;
use clap::Parser;
use log::{error, info};
use simple_logger::SimpleLogger;
use std::io::Error;
use std::process::Command;

/// Struct to hold command-line arguments
#[derive(Parser, Debug)]
#[clap()]
struct Args {
    /// The command to run and its arguments
    #[clap()]
    command: Vec<String>,
}

fn run_command(cmd: &[String]) -> Result<(), Error> {
    let (command, args) = cmd.split_at(1);
    let output = Command::new(&command[0]).args(args).output()?;

    if !output.status.success() {
        let time = Local::now();
        error!(
            "Command failed with exit code {} at {}",
            output.status, time
        );
        return Err(Error::new(std::io::ErrorKind::Other, "Command failed"));
    }

    info!("Command executed successfully");

    Ok(())
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Args::parse();

    loop {
        match run_command(&args.command) {
            Ok(_) => (),
            Err(e) => {
                error!("{}", e);
            }
        }
    }
}
