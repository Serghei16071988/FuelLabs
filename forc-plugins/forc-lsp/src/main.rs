//! A simple `forc` plugin for starting the sway language server.
//!
//! Once installed and available via `PATH`, can be executed via `forc lsp`.

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = "forc-lsp",
    about = "Forc plugin for the Sway LSP (Language Server Protocol) implementation.",
    version
)]
pub struct Command {
    /// The port of the socket to connect to.
    #[clap(long)]
    pub socket: String,
}

#[tokio::main]
async fn main() {
    let Command { socket } = Command::parse();

    sway_lsp::start(socket).await
}
