pub mod ast;
pub mod cst;
mod cst_parser;
pub mod features;
pub mod lexer;
pub mod lsp;
pub mod morphology;
pub mod parser;
pub mod semantic;
pub mod syntax;

pub use parser::{ParserOptions, parse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    lsp::server::run().await
}
