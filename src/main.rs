use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gos-sign", about = "GrapheneOS release signing")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate signing keys
    GenerateKeys,
    /// Sign an OTA update package
    SignOta,
    /// Sign a factory image
    SignFactory,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::GenerateKeys => {
            println!("gos-sign: generating signing keys");
        }
        Command::SignOta => {
            println!("gos-sign: signing OTA package");
        }
        Command::SignFactory => {
            println!("gos-sign: signing factory image");
        }
    }
}
