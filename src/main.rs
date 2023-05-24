use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    //create a file
    let mut file = File::create("output.txt").expect("TODO: panic message");

    //get git log of a repository
    let output = Command::new("git")
        .args(["log", "--pretty=format:%H"])
        .output()
        .expect("failed to execute process Git");

    //parse the output
    let output = String::from_utf8_lossy(&output.stdout);
    let commits = output.split('\n');

    //write the output to a file
    for commit in commits {
        writeln!(file, "{}", commit).expect("TODO: panic message");
    }
}