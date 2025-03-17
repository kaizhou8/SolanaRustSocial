//! 状态管理模块

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

use crate::{error::SolanaRustError, models::{Content, User}};

/// 账户状态枚举
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum AccountState {
    /// 未初始化状态
    Uninitialized,
    /// 用户账户
    User(User),
    /// 内容账户
    Content(Content),
}

impl AccountState {
    /// 从账户信息中加载状态
    pub fn load(account_info: &AccountInfo) -> Result<Self, ProgramError> {
        let data = account_info.try_borrow_data()?;
        Self::try_from_slice(&data).map_err(|_| SolanaRustError::DeserializationError.into())
    }

    /// 保存状态到账户
    pub fn save(&self, account_info: &AccountInfo) -> Result<(), ProgramError> {
        let mut data = account_info.try_borrow_mut_data()?;
        let serialized = self.try_to_vec().map_err(|_| SolanaRustError::SerializationError)?;
        
        if serialized.len() > data.len() {
            return Err(ProgramError::AccountDataTooSmall);
        }
        
        data[..serialized.len()].copy_from_slice(&serialized);
        Ok(())
    }
}

/// 派生用户账户地址
pub fn derive_user_address(owner: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"user", owner.as_ref()],
        program_id,
    )
}

/// 派生内容账户地址
pub fn derive_content_address(
    author: &Pubkey,
    content_seed: &[u8],
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"content", author.as_ref(), content_seed],
        program_id,
    )
}

/// 派生代币信息账户地址
pub fn derive_token_address(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"token"],
        program_id,
    )
}

/// 派生用户代币账户地址
pub fn derive_token_account_address(owner: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[b"token_account", owner.as_ref()],
        &PROGRAM_ID.parse::<Pubkey>().unwrap(),
    )
}