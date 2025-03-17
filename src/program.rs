//! Solana程序模块

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::error::SolanaRustError;

/// 程序ID
pub const PROGRAM_ID: &str = "SocialTokenProgram1111111111111111111111111";

/// 程序入口点
entrypoint!(process_instruction);

/// 指令枚举
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum SocialInstruction {
    /// 创建用户
    /// 0. `[signer]` 用户账户
    /// 1. `[writable]` 用户数据账户
    CreateUser { name: String },

    /// 更新用户信息
    /// 0. `[signer]` 用户账户
    /// 1. `[writable]` 用户数据账户
    UpdateUser { name: String, bio: String },

    /// 发布内容
    /// 0. `[signer]` 用户账户
    /// 1. `[writable]` 内容数据账户
    /// 2. `[]` 用户数据账户
    PostContent { text: String },

    /// 点赞内容
    /// 0. `[signer]` 用户账户
    /// 1. `[writable]` 内容数据账户
    LikeContent,

    /// 关注用户
    /// 0. `[signer]` 关注者账户
    /// 1. `[writable]` 关注者数据账户
    /// 2. `[writable]` 被关注者数据账户
    FollowUser,
}

/// 处理指令函数
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SocialInstruction::try_from_slice(instruction_data)
        .map_err(|_| SolanaRustError::InvalidInstruction)?;

    match instruction {
        SocialInstruction::CreateUser { name } => {
            // 实现创建用户逻辑
            Ok(())
        }
        SocialInstruction::UpdateUser { name, bio } => {
            // 实现更新用户逻辑
            Ok(())
        }
        SocialInstruction::PostContent { text } => {
            // 实现发布内容逻辑
            Ok(())
        }
        SocialInstruction::LikeContent => {
            // 实现点赞内容逻辑
            Ok(())
        }
        SocialInstruction::FollowUser => {
            // 实现关注用户逻辑
            Ok(())
        }
    }
}