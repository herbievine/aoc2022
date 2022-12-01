use clap::{value_parser, Arg, Command};

pub fn commands() -> Command {
    Command::new("aoc")
        .about("A CLI to build and run the Advent of Code projects.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run").about("Run a specific day.").arg(
                Arg::new("day")
                    .value_parser(value_parser!(u16))
                    .required(true),
            ),
        )
}
