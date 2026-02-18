extern crate log;

pub mod setloglevel;

use clap::Parser;

fn do_panic() {
    log::debug!("Setting a global panic handler");
    better_panic::Settings::auto()
        .most_recent_first(false)
        .lineno_suffix(true)
        .verbosity(better_panic::Verbosity::Full)
        .install();
}

pub fn main() {
    let cli = Cli::parse();
    setloglevel::setloglevel(&cli);
    do_panic();
    log::debug!("BITMASHER tool context initialized ...");
}

#[derive(Parser, Clone, Debug)]
#[clap(name = "bitmasher")]
#[clap(author = "Vladimir Ulogov <vladimir@ulogov.us>")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(
    about = "ENCRYPT/DECRYPT with Bitmasher",
    long_about = "Bitmasher encryption and decryption tool"
)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}
