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

fn main() {
    // Process arguments
    let opt = Opt::from_args();

    if opt.command.is_empty() {
        println!("Need to supply a command.");
        return;
    }

    do_watchdiff(opt);
}

fn do_watchdiff(opt: Opt) {
    // Setup command and arguments
    let mut raw = Command::new(&opt.command[0]);
    let cmd = raw.args(&opt.command[1..]);

    // Run initial command
    let mut out = run_command(cmd);
    println!("{}", out);

    loop {
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
    let local = Local::now();
    let banner = format!("Diff at {}", local.to_string());
    println!("{}", banner.bold().underline());
    let orig = a.split('\n').collect::<Vec<&str>>();
    let new = b.split('\n').collect::<Vec<&str>>();

    let orig_max = orig.len() - 1;
    let new_max = new.len() - 1;
    let mut orig_idx = 0;
    let mut new_idx = 0;

    'check: loop {
        if orig_idx == orig_max && new_idx == new_max {
            break;
        } else if orig_idx == orig_max && new_idx < new_max {
            print_all(&new[new_idx..new_max], PrintType::Add);
            break;
        } else if new_idx == new_max && orig_idx < orig_max {
            print_all(&orig[orig_idx..orig_max], PrintType::Del);
            break;
        } else if orig[orig_idx] == new[new_idx] {
            print_same(orig[orig_idx]);
            orig_idx += 1;
            new_idx += 1;
            continue;
        } else {
            let tmp = new_idx;
            for i in tmp..new_max {
                if orig[orig_idx] == new[i] {
                    print_add(new[new_idx]);
                    new_idx += 1;
                    continue 'check;
                }
            }
            let tmp = orig_idx;
            for i in tmp..orig_max {
                if new[new_idx] == orig[i] {
                    print_del(orig[orig_idx]);
                    orig_idx += 1;
                    continue 'check;
                }
            }
        }

        print_del(orig[orig_idx]);
        print_add(new[new_idx]);

        orig_idx += 1;
        new_idx += 1;
    }
}

fn print_all(a: &[&str], print: PrintType) {
    match print {
        PrintType::Add => a.iter().map(|a| print_add(a)).collect(),
        PrintType::Del => a.iter().map(|a| print_del(a)).collect(),
    }
}

fn print_add(a: &str) {
    let b = format!(" + {}", a);
    println!("{}", b.green());
}

fn print_del(a: &str) {
    let b = format!(" - {}", a);
    println!("{}", b.red());
}

fn print_same(a: &str) {
    println!("   {}", a);
}
