use clap::{Command, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Install {
        #[clap(value_parser)]
        product: Option<String>
    }
}

fn main() {
    let cli = Cli::parse();
   
    match &cli.command {
        Commands::Install { product } => {
            println!("ainda n me chame de bb {:?}", product);
        }
    }
}
