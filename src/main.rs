use chrono::prelude::*;
use colored::*;
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

enum PrintType {
    Add,
    Del,
}

const BASH: &str = "/bin/bash";
const C: &str = "-c";

fn main() {
    // Process arguments
    let opt = Opt::from_args();

    // Require a command to run
    if opt.command.is_empty() {
        println!("Need to supply a command.");
        return;
    }

    // Run command
    do_watchdiff(opt);
}

fn do_watchdiff(opt: Opt) {
    // Print origin time
    let local = Local::now();
    let banner = format!("Origin at {}", local);
    println!("{}", banner.bold().underline());

    // Setup command and arguments
    let mut raw = Command::new(BASH);
    let mut args = vec![C];
    for c in &opt.command {
        args.push(c);
    }
    let cmd = raw.args(&args);

    // Run initial command
    let mut out = run_command(cmd);
    println!("{}", out);

    loop {
        // Re-setup command and arguments
        let mut raw = Command::new(BASH);
        let cmd = raw.args(&args);

        // Run command update
        let diff = run_command(cmd);

        // Compare with previous output
        if out != diff {
            print_diff(&out, &diff);
            if !opt.permament {
                out = diff;
            }
        }

        // Sleep for specified delay time
        thread::sleep(time::Duration::from_secs(opt.delay));
    }
}

fn run_command(cmd: &mut Command) -> String {
    // Execute the command and return the stdout buffer as a string
    let out = cmd.output().unwrap();
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn print_diff(a: &str, b: &str) {
    // Print diff time
    let local = Local::now();
    let banner = format!("Diff at {}", local);
    println!("{}", banner.bold().underline());

    // Split output into lines
    let orig = a.split('\n').collect::<Vec<&str>>();
    let new = b.split('\n').collect::<Vec<&str>>();

    // Calculate the max index values
    let orig_max = orig.len() - 1;
    let new_max = new.len() - 1;

    // Instantiate counters
    let mut orig_idx = 0;
    let mut new_idx = 0;

    // Iterate through each index of the orig and new lists
    'check: loop {
        if orig_idx == orig_max && new_idx == new_max {
            // If we've reached the end of both lists, we're done
            break;
        } else if orig_idx == orig_max && new_idx < new_max {
            // If we've reached the end of the original, everything else was added
            print_all(&new[new_idx..new_max], PrintType::Add);
            break;
        } else if new_idx == new_max && orig_idx < orig_max {
            // If we've reached the end of the new, everything else was removed
            print_all(&orig[orig_idx..orig_max], PrintType::Del);
            break;
        } else if orig[orig_idx] == new[new_idx] {
            // If both values are identical, there was no change
            print_same(orig[orig_idx]);
            orig_idx += 1;
            new_idx += 1;
            continue;
        } else {
            // Iterate through the rest of the new list looking for the current old to identify an added item in new
            let tmp = new_idx;
            for i in tmp..new_max {
                if orig[orig_idx] == new[i] {
                    print_add(new[new_idx]);
                    new_idx += 1;
                    continue 'check;
                }
            }

            // Iterate through the rest of the orig list looking for the current new to identify a removed item in orig
            let tmp = orig_idx;
            for i in tmp..orig_max {
                if new[new_idx] == orig[i] {
                    print_del(orig[orig_idx]);
                    orig_idx += 1;
                    continue 'check;
                }
            }
        }

        // If nothing else was detected, the current index was both removed from orig and added in new
        print_del(orig[orig_idx]);
        print_add(new[new_idx]);

        orig_idx += 1;
        new_idx += 1;
    }
}

fn print_all(a: &[&str], print: PrintType) {
    // Depending on Add or Del, iterate through the slice and print each line
    match print {
        PrintType::Add => a.iter().map(|a| print_add(a)).collect(),
        PrintType::Del => a.iter().map(|a| print_del(a)).collect(),
    }
}

fn print_add(a: &str) {
    // Print an added line
    let b = format!(" + {}", a);
    println!("{}", b.green());
}

fn print_del(a: &str) {
    // Print a removed line
    let b = format!(" - {}", a);
    println!("{}", b.red());
}

fn print_same(a: &str) {
    // Print a line
    println!("   {}", a);
}
