# Git Auto Logger

The Git Auto Logger is a command-line tool written in Rust. It fetches the git commit history and writes it to an output
file with additional filters such as commit author, number of commits to show, and more.

This generated file can help you analyze the commit history and extract insightful information from your repository.

## Usage

This command will retrieve the last two commits (specified by --count 2) made by "john@doe.com" (specified by --email
john@doe.com) within the last 1 day (specified by --days 1).

Command Line Options
--email: Filters the logs by the provided email. If left empty, logs of all authors are retrieved.
--count: The number of commits to retrieve, default is 10.
—-days: The number of days to go back while retrieving the logs, default is 1.
--debug: Prints debug information.

Logs are written to the output.txt file in the root directory of the project.

## Development

This project uses Rust and external crates such as clap for command-line argument parsing, and logs for logging.
You can build and test the application using the standard Rust commands:

```cargo
cargo build
cargo test
```

## License

This project is licensed under the MIT License—see the [LICENSE](./LICENSE.md) file for details

## TODOs for the future

- [ ] Add tests
- [ ] Add configuration file
- [ ] Add CD/CI

## Buy me a coffee

If you like this project, consider 
[Buy me a coffee](https://www.buymeacoffee.com/PCtzonoes)

