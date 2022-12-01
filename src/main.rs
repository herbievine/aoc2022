mod cli;
mod days;
mod utils;

use days::*;

fn main() {
    let commands = cli::commands();
    let day: u16;

    match commands.get_matches().subcommand() {
        Some(("run", sub_matches)) => {
            day = *sub_matches.get_one::<u16>("day").unwrap();
        }
        _ => unreachable!(),
    }

    match day {
        1 => {
            day01::a("./resources/day01.txt");
            day01::b("./resources/day01.txt");
        }
        x => {
            println!("Day {} is not yet implemented", x);
            panic!();
        }
    }
}
