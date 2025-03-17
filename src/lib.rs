//! Solana社交媒体应用库

pub mod error;
pub mod models;
pub mod program;
pub mod state;
pub mod token;
pub mod utils;

/// 重新导出常用模块
pub use error::SolanaRustError;
pub use models::*;
pub use program::*;
pub use state::*;
pub use token::*;