use clap::{Parser, Subcommand, command};
use kvs::Result;
// Initialize clap for argument parsing


// Clap's configuration
#[derive(Parser)]
#[command(name = "kvs", version, about, author)]
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

fn main() -> Result<()> {

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
    
    panic!()
}
