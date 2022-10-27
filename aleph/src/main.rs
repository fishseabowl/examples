use aleph_bft::{run_session, NodeIndex, Terminator};
use clap::Parser;
use tokio;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Index of the node
    #[clap(long, value_parser)]
    id: usize,

    /// Ports
    #[clap(long, value_parser, value_delimiter = ',')]
    ports: Vec<usize>,

    /// Number of items to be ordered
    #[clap(long, value_parser)]
    n_data: u32,

    /// Number of the first created item
    #[clap(default_value = "0", long, value_parser)]
    n_starting: u32,

    /// Indices of nodes having stalling DataProviders
    #[clap(default_value = "", long, value_parser, value_delimiter = ',')]
    stalled: Vec<usize>,

    /// Should the node crash after finalizing its items
    #[clap(long, value_parser)]
    crash: bool,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init_timed();
}
