use std::process::Command;
use std::{thread, time};

fn main() {
    let mut list_dir = Command::new("ls");

    let orig = list_dir.output().unwrap();
    let out = String::from_utf8_lossy(&orig.stdout);
    println!("{}", out);

    loop {
        let compare = list_dir.output().unwrap();
        let diff = String::from_utf8_lossy(&compare.stdout);

        if out != diff {
            println!("{}", diff);
            break;
        }

        thread::sleep(time::Duration::from_secs(1));
    }

}
