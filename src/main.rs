use clap::Parser;
use fsdgen::{create_layer_slice, Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New(new)) => {
            let layer = new.layer.unwrap();
            create_layer_slice(&layer);
        }
        None => println!("Please provide command."),
    }
}
