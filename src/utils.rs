//! 工具函数模块

use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::instruction::{AccountMeta, Instruction};
use std::error::Error;

use crate::models::{User, Content};
use crate::program::{SocialInstruction, PROGRAM_ID};
use crate::state::{derive_user_address, derive_content_address, AccountState};

/// 创建用户函数
pub async fn create_user(
    rpc_client: &RpcClient,
    keypair: &Keypair,
    name: String,
) -> Result<(), Box<dyn Error>> {
    let program_id = PROGRAM_ID.parse::<Pubkey>()?;
    let (user_pda, _) = derive_user_address(&keypair.pubkey(), &program_id);
    
    // 创建指令
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(keypair.pubkey(), true),
            AccountMeta::new(user_pda, false),
        ],
        data: SocialInstruction::CreateUser { name }.try_to_vec()?,
    };
    
    // 创建并发送交易
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair.pubkey()),
        &[keypair],
        rpc_client.get_latest_blockhash()?,
    );
    
    rpc_client.send_and_confirm_transaction(&transaction)?;
    println!("用户创建成功！");
    
    Ok(())
}

/// 发布内容函数
pub async fn post_content(
    rpc_client: &RpcClient,
    keypair: &Keypair,
    text: String,
) -> Result<(), Box<dyn Error>> {
    let program_id = PROGRAM_ID.parse::<Pubkey>()?;
    let (user_pda, _) = derive_user_address(&keypair.pubkey(), &program_id);
    
    // 生成内容种子
    let content_seed = format!("{}-{}", keypair.pubkey(), text.len()).into_bytes();
    let (content_pda, _) = derive_content_address(&keypair.pubkey(), &content_seed, &program_id);
    
    // 创建指令
    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(keypair.pubkey(), true),
            AccountMeta::new(content_pda, false),
            AccountMeta::new_readonly(user_pda, false),
        ],
        data: SocialInstruction::PostContent { text }.try_to_vec()?,
    };
    
    // 创建并发送交易
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair.pubkey()),
        &[keypair],
        rpc_client.get_latest_blockhash()?,
    );
    
    rpc_client.send_and_confirm_transaction(&transaction)?;
    println!("内容发布成功！");
    
    Ok(())
}

/// 获取用户信息函数
pub async fn get_user(
    rpc_client: &RpcClient,
    address: &str,
) -> Result<Option<User>, Box<dyn Error>> {
    let program_id = PROGRAM_ID.parse::<Pubkey>()?;
    let user_pubkey = address.parse::<Pubkey>()?;
    let (user_pda, _) = derive_user_address(&user_pubkey, &program_id);
    
    // 获取账户数据
    match rpc_client.get_account_data(&user_pda) {
        Ok(data) => {
            // 反序列化账户状态
            let account_state = AccountState::try_from_slice(&data)?;
            
            match account_state {
                AccountState::User(user) => Ok(Some(user)),
                _ => Ok(None),
            }
        },
        Err(_) => Ok(None),
    }
}

/// 获取内容信息函数
pub async fn get_content(
    rpc_client: &RpcClient,
    content_id: &str,
) -> Result<Option<Content>, Box<dyn Error>> {
    let content_pubkey = content_id.parse::<Pubkey>()?;
    
    // 获取账户数据
    match rpc_client.get_account_data(&content_pubkey) {
        Ok(data) => {
            // 反序列化账户状态
            let account_state = AccountState::try_from_slice(&data)?;
            
            match account_state {
                AccountState::Content(content) => Ok(Some(content)),
                _ => Ok(None),
            }
        },
        Err(_) => Ok(None),
    }
}