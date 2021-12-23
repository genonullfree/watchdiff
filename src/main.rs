use chrono::prelude::*;
use std::process::Command;
use std::{thread, time};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub struct Opt {
    /// Delay between runs in seconds
    #[structopt(short, long, default_value = "2")]
    delay: u64,

    /// Compare to the initial output (permament mode)
    #[structopt(short, long)]
    permament: bool,

    /// Command to run
    command: Vec<String>,
}

fn main() {
    // Process arguments
    let opt = Opt::from_args();

    if opt.command.is_empty() {
        println!("Need to supply a command.");
        return;
    }

    // Setup command and arguments
    let mut raw = Command::new(&opt.command[0]);
    let cmd = raw.args(&opt.command[1..]);

    let mut out = run_command(cmd);
    println!("{}", out);

    loop {
        let diff = run_command(cmd);

        if out != diff {
            let local = Local::now();
            println!("Diff at {}", local.to_string());
            println!("{}", diff);
            if !opt.permament {
                out = diff;
            }
        }

        thread::sleep(time::Duration::from_secs(opt.delay));
    }
}

fn run_command(cmd: &mut Command) -> String {
    // Execute the command and return the stdout buffer as a string
    let out = cmd.output().unwrap();
    String::from_utf8_lossy(&out.stdout).to_string()
}
