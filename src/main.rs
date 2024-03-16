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
use std::path::PathBuf;
use std::process::{Command, Output};

/// Struct to hold command-line arguments
#[derive(Parser, Debug)]
#[clap()]
struct Args {
    /// The command to run and its arguments
    #[clap()]
    command: Vec<String>,
}

fn run_command(command: PathBuf, args: &[String]) -> Result<(), Error> {
    let output = Command::new(&command).args(args).output()?;

    // If the command exited with an error, we need to log the stderr to a file
    if !output.status.success() {
        error!("Command failed with exit code {}", output.status);

        // Make sure that if command is a path with directories, we only get the
        // last part of it
        let command_name = command
            .file_name()
            .ok_or(Error::new(
                std::io::ErrorKind::Other,
                "Could not get command name",
            ))?
            .to_str()
            .ok_or(Error::new(
                std::io::ErrorKind::Other,
                "Could not convert command name to string",
            ))?;

        write_to_log_file(command_name, &output, Local::now().into())?;
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

    // First, check if the command exists in the PATH or the current directory.
    // If it's found, add an absolute path to the command.
    let command = &args.command[0];
    let command_path = match which::which(&command) {
        Ok(path) => path,
        Err(_) => {
            let current_dir = std::env::current_dir().unwrap();
            let local_path = current_dir.join(&command);
            if local_path.exists() {
                local_path
            } else {
                // This panic will happen right as the command is run, so it's
                // ok
                error!("Command '{}' not found (on PATH or relative to where this command was run)", command);
                return;
            }
        }
    };

    // The args are the rest of the command-line arguments
    let args = &args.command[1..];

    loop {
        match run_command(command_path.clone(), args) {
            Ok(_) => (),
            Err(e) => {
                error!("{}", e);
            }
        }
    }
}
