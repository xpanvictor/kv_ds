use std::collections::hash_map::Keys;
use clap::{Parser, Subcommand, command};
// Initialize clap for argument parsing


// Clap's configuration
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Args {
    /// Sub commands for kvs
    #[clap(subcommand)]
    cmd: Subcommands
}


#[derive(Subcommand)]
enum Subcommands {
    /// Set a value to key
    Set {
        /// Name of key to store
        key: String,
        /// Value of key to store
        value: String
    },
    /// Retrieves value of key
    Get {
        /// Name of the key
        key: String
    },
    /// Remove a key from the store
    Rm {
        /// key to remove
        key: String
    }
}

fn main() {

    let env_vars = Args::parse();

    match env_vars.cmd {
        Subcommands::Get {key} => {
            eprintln!("unimplemented");
            panic!();
        }
        Subcommands::Set {key, value} => {
            eprintln!("unimplemented");
            panic!();
        }
        Subcommands::Rm {key} => {
            eprintln!("unimplemented");
            panic!();
        }
        _ => {
            panic!()
        }
    }


}
