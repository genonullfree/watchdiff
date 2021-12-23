use chrono::prelude::*;
use std::process::Command;
use std::{thread, time};
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub struct Opt {
    /// Delay between runs in seconds
    #[structopt(short, long, default_value = "2")]
    delay: u64,

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
    let mut run_raw = Command::new(&opt.command[0]);
    let run_cmd = run_raw.args(&opt.command[1..]);

    let orig = run_cmd.output().unwrap();
    let out = String::from_utf8_lossy(&orig.stdout);
    println!("{}", out);

    loop {
        let compare = run_cmd.output().unwrap();
        let diff = String::from_utf8_lossy(&compare.stdout);

        if out != diff {
            let local = Local::now();
            println!("Diff at {}", local.to_string());
            println!("{}", diff);
            break;
        }

        thread::sleep(time::Duration::from_secs(opt.delay));
    }
}
