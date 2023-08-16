#[macro_use]
extern crate log;

use std::fs::File;
use std::io::Write;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Email of the committer to filter by
	#[arg(short, long, default_value = "")]
	email: String,
	/// How many commits to show
	#[arg(short, long, default_value = "10")]
	count: usize,
	///How many days to go back
	#[arg(short, long, default_value = "1")]
	days: usize,
	/// Print debug information
	#[arg(long)]
	debug: bool,
}

fn main() {
	let Args { email, count, days, debug } = Args::parse();

	// Initialize the logger
	if debug {
		std::env::set_var("RUST_LOG", "debug");
	}
	env_logger::init();

	debug!("Received arguments: {:?}", Args::parse());

	// Create a file
	let mut file = File::create("output.txt").expect("Failed to create the output file.");
	if let Err(e) = writeln!(file, "This is a summary of recent code changes. \
	Each entry includes the author's name and the short message as a description of the change they made,\
	please add more info to these changes useful for non-developers, can be a bit casual:") {
		eprintln!("Failed to write to file: {}", e);
	}

	let format_string = format!(
		"%cI%n{}%n%B",
		if email.is_empty() { "%an" } else { "%ae" }
	);

	let mut process = std::process::Command::new("git");
	process.args(&["log", "--no-show-signature", &("--pretty=format:".to_string() + &format_string)]);
	process.args(&["--since", &format!("{} days ago", days)]); // Limit the number of days
	process.args(&["-n", &count.to_string()]); // Limit the number of commits

	if !email.is_empty() {
		process.args(&["--perl-regexp", "--author", &email]);
	}

	let process_output = process.output().expect("Failed to execute git log.");
	let output = String::from_utf8_lossy(&process_output.stdout);
	let logs = output.split("\n\n\n");

// Write the output to a file
	for log in logs {
		let mut lines = log.lines();
		let _ = lines.next();  // Skip the commit date
		let logged_email = lines.next().unwrap_or_default();
		let message = lines.collect::<Vec<&str>>().join("\n");

		if let Err(e) = writeln!(file, "Here is the next change:") {
			eprintln!("Failed to write to file: {}", e);
		}

		if email.is_empty() || logged_email == email {
			if let Err(e) = writeln!(file, "{} did {}", logged_email, message) {
				eprintln!("Failed to write to file: {}", e);
			}
		}
	}

	if let Err(e) = writeln!(file, "Based on these changes, what features have been worked on recently?\
	Be concise since this is for a standup meeting, so one small line per feature is best") {
		eprintln!("Failed to write to file: {}", e);
	}

	info!("Logs written to output.txt");
}
