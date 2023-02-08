use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Neha Barde",
    about = "A Redaction micro service"
)]
struct Cli {
    #[clap(subcommand)]
    //command: Option<Commands>,
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Neha Barde")]
    Start,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Start) => {
            redaction::data();
        }
        None => println!("No subcommand was used"),
    }
}
