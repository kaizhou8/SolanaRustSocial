//! 代币服务模块

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::transaction::Transaction;
use std::error::Error;

use crate::program::{SocialInstruction, PROGRAM_ID};
use crate::state::{AccountState, derive_token_address};

/// 代币信息结构体
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TokenInfo {
    /// 代币名称
    pub name: String,
    /// 代币符号
    pub symbol: String,
    /// 总供应量
    pub total_supply: u64,
    /// 精度（小数位数）
    pub decimals: u8,
    /// 创建时间戳
    pub created_at: u64,
}

/// 代币账户结构体
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TokenAccount {
    /// 所有者公钥
    pub owner: Pubkey,
    /// 余额
    pub balance: u64,
    /// 是否被冻结
    pub is_frozen: bool,
    /// 创建时间戳
    pub created_at: u64,
    /// 更新时间戳
    pub updated_at: u64,
}

/// 代币指令枚举
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum TokenInstruction {
    /// 初始化代币
    /// 0. `[signer]` 创建者账户
    /// 1. `[writable]` 代币信息账户
    InitializeToken {
        name: String,
        symbol: String,
        total_supply: u64,
        decimals: u8,
    },
    
    /// 创建代币账户
    /// 0. `[signer]` 所有者账户
    /// 1. `[writable]` 代币账户
    CreateTokenAccount,
    
    /// 转账代币
    /// 0. `[signer]` 发送者账户
    /// 1. `[writable]` 发送者代币账户
    /// 2. `[writable]` 接收者代币账户
    Transfer { amount: u64 },
    
    /// 奖励代币（用于内容创作和互动）
    /// 0. `[signer]` 系统账户
    /// 1. `[writable]` 接收者代币账户
    /// 2. `[]` 接收者用户账户
    Reward { amount: u64, reason: String },
}

/// 初始化代币函数
pub async fn initialize_token(
    rpc_client: &RpcClient,
    keypair: &Keypair,
    name: String,
    symbol: String,
    total_supply: u64,
    decimals: u8,
) -> Result<(), Box<dyn Error>> {
    let program_id = PROGRAM_ID.parse::<Pubkey>()?;
    let (token_pda, _) = derive_token_address(&program_id);
    
    // 创建指令
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(keypair.pubkey(), true),
            AccountMeta::new(token_pda, false),
        ],
        data: TokenInstruction::InitializeToken {
            name,
            symbol,
            total_supply,
            decimals,
        }.try_to_vec()?,
    };
    
    // 创建并发送交易
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair.pubkey()),
        &[keypair],
        rpc_client.get_latest_blockhash()?,
    );
    
    rpc_client.send_and_confirm_transaction(&transaction)?;
    println!("代币初始化成功！");
    
    Ok(())
}

/// 创建代币账户函数
pub async fn create_token_account(
    rpc_client: &RpcClient,
    keypair: &Keypair,
) -> Result<(), Box<dyn Error>> {
    let program_id = PROGRAM_ID.parse::<Pubkey>()?;
    let (token_account_pda, _) = derive_token_address(&keypair.pubkey());
    
    // 创建指令
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(keypair.pubkey(), true),
            AccountMeta::new(token_account_pda, false),
        ],
        data: TokenInstruction::CreateTokenAccount.try_to_vec()?,
    };
    
    // 创建并发送交易
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair.pubkey()),
        &[keypair],
        rpc_client.get_latest_blockhash()?,
    );
    
    rpc_client.send_and_confirm_transaction(&transaction)?;
    println!("代币账户创建成功！");
    
    Ok(())
}

/// 转账代币函数
pub async fn transfer_token(
    rpc_client: &RpcClient,
    keypair: &Keypair,
    recipient: &Pubkey,
    amount: u64,
) -> Result<(), Box<dyn Error>> {
    let program_id = PROGRAM_ID.parse::<Pubkey>()?;
    let (sender_token_account, _) = derive_token_address(&keypair.pubkey());
    let (recipient_token_account, _) = derive_token_address(recipient);
    
    // 创建指令
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(keypair.pubkey(), true),
            AccountMeta::new(sender_token_account, false),
            AccountMeta::new(recipient_token_account, false),
        ],
        data: TokenInstruction::Transfer { amount }.try_to_vec()?,
    };
    
    // 创建并发送交易
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair.pubkey()),
        &[keypair],
        rpc_client.get_latest_blockhash()?,
    );
    
    rpc_client.send_and_confirm_transaction(&transaction)?;
    println!("代币转账成功！");
    
    Ok(())
}

/// 奖励代币函数（用于内容创作和互动）
pub async fn reward_token(
    rpc_client: &RpcClient,
    keypair: &Keypair,
    recipient: &Pubkey,
    amount: u64,
    reason: String,
) -> Result<(), Box<dyn Error>> {
    let program_id = PROGRAM_ID.parse::<Pubkey>()?;
    let (recipient_token_account, _) = derive_token_address(recipient);
    let (recipient_user_account, _) = derive_token_address(recipient);
    
    // 创建指令
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(keypair.pubkey(), true),
            AccountMeta::new(recipient_token_account, false),
            AccountMeta::new_readonly(recipient_user_account, false),
        ],
        data: TokenInstruction::Reward { amount, reason }.try_to_vec()?,
    };
    
    // 创建并发送交易
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair.pubkey()),
        &[keypair],
        rpc_client.get_latest_blockhash()?,
    );
    
    rpc_client.send_and_confirm_transaction(&transaction)?;
    println!("代币奖励发放成功！");
    
    Ok(())
}