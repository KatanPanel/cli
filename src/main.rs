mod commands;

use clap::Command;

fn main() -> std::io::Result<()> {
    let command = Command::new("katan")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("version"));

    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("version", _)) => commands::version::version(),
        _ => unreachable!(),
    }
}
