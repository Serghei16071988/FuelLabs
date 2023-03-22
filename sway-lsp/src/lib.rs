#![recursion_limit = "256"]

mod capabilities;
pub mod config;
mod core;
pub mod error;
pub mod server;
mod traverse;
pub mod utils;

use server::Backend;
use tokio::net::TcpStream;
// use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use tower_lsp::{LspService, Server};

pub async fn start(port: String) {
    // let stdin = tokio::io::stdin();
    // let stdout = tokio::io::stdout();

    // We presume that the client has opened the TCP port and is waiting
    // for us to connect. This is the connection pattern used by clients
    // built with vscode-langaugeclient.
    let stream = TcpStream::connect(format!("127.0.0.1:{port}"))
        .await
        .unwrap();
    let (read, write) = tokio::io::split(stream);

    // let (read, write) = (read.compat(), write.compat_write());
    // let (read, write) = (read.into_inner(), write.into_inner());
    // let (read, write) = (read., write.compat_write()

    let (service, socket) = LspService::build(Backend::new)
        .custom_method("sway/show_ast", Backend::show_ast)
        .custom_method("textDocument/inlayHint", Backend::inlay_hints)
        .finish();
    Server::new(read, write, socket).serve(service).await;
}
