//! 错误处理模块

use solana_program::program_error::ProgramError;
use thiserror::Error;

/// 应用程序错误枚举
#[derive(Error, Debug)]
pub enum SolanaRustError {
    #[error("无效的指令")]
    InvalidInstruction,

    #[error("无效的用户数据")]
    InvalidUserData,

    #[error("无效的内容数据")]
    InvalidContentData,

    #[error("用户不存在")]
    UserNotFound,

    #[error("内容不存在")]
    ContentNotFound,

    #[error("权限不足")]
    InsufficientPermission,

    #[error("序列化错误")]
    SerializationError,

    #[error("反序列化错误")]
    DeserializationError,

    #[error("未知错误")]
    Unknown,
}

impl From<SolanaRustError> for ProgramError {
    fn from(e: SolanaRustError) -> Self {
        ProgramError::Custom(e as u32)
    }
}