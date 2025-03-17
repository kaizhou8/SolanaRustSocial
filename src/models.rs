//! 数据模型模块

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use std::time::{SystemTime, UNIX_EPOCH};

/// 用户模型
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct User {
    /// 用户公钥
    pub pubkey: Pubkey,
    /// 用户名
    pub name: String,
    /// 用户简介
    pub bio: String,
    /// 关注者数量
    pub followers_count: u64,
    /// 关注数量
    pub following_count: u64,
    /// 创建时间戳
    pub created_at: u64,
    /// 更新时间戳
    pub updated_at: u64,
}

impl User {
    pub fn new(pubkey: Pubkey, name: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            pubkey,
            name,
            bio: String::new(),
            followers_count: 0,
            following_count: 0,
            created_at: now,
            updated_at: now,
        }
    }
}

/// 内容模型
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Content {
    /// 内容ID
    pub id: Pubkey,
    /// 作者公钥
    pub author: Pubkey,
    /// 内容文本
    pub text: String,
    /// 点赞数量
    pub likes_count: u64,
    /// 评论数量
    pub comments_count: u64,
    /// 创建时间戳
    pub created_at: u64,
}

impl Content {
    pub fn new(id: Pubkey, author: Pubkey, text: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id,
            author,
            text,
            likes_count: 0,
            comments_count: 0,
            created_at: now,
        }
    }
}