
use clap::{arg, Command};
// Initialize clap for argument parsing

fn main() {
    let matches = Command::new("kvs")
        .version("1.0")
        .about("Key Value store")
        .arg(arg!(-s --set <String>))
        .arg(arg!(-g --get <String>))
        .arg(arg!(-r --remove <String>))
        .get_matches();
}
