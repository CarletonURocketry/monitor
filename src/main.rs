/// Example usage:
/// ```
/// $ monitor ls -- -l
/// ```
/// This will run the `ls -l` command repeatedly until it fails. If the command
/// fails, an error message will be logged. Note that -- is used to separate the
/// command from its arguments.
use chrono::{DateTime, Local};
use clap::Parser;
use log::{error, info};
use simple_logger::SimpleLogger;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{Error, Write};
use std::process::{Command, Output};

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
        let time: DateTime<Local> = Local::now();
        error!(
            "Command failed with exit code {} at {}",
            output.status, time
        );

        // Make sure that if command is a path with directories, we only get the
        // last part of it
        let command_name = match command[0].split('/').last() {
            Some(c) => c,
            None => &command[0],
        };

        write_to_log_file(&command_name, &output, time.into())?;
        return Err(Error::new(std::io::ErrorKind::Other, "Command failed"));
    }

    info!("Command executed successfully");

    Ok(())
}

/// Write the stderr of a command to a log file
fn write_to_log_file(command: &str, output: &Output, time: DateTime<Local>) -> Result<(), Error> {
    let home_dir = dirs::home_dir().expect("Could not get home directory");
    let log_dir = home_dir.join(".local/share/monitor");
    create_dir_all(&log_dir)?;

    // Example filename: failing-program-2024-03-16-17-50-40-843.log
    let log_file_name = format!("{}-{}.log", command, time.format("%Y-%m-%d-%H-%M-%S-%3f"));
    let log_file_path = log_dir.join(log_file_name);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(log_file_path)?;

    file.write_all(&output.stderr)?;

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
