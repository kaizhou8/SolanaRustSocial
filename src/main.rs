use clap::{Parser, Subcommand};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "https://api.devnet.solana.com")]
    url: String,
}

#[derive(Subcommand)]
enum Commands {
    /// 创建新用户
    CreateUser {
        #[arg(short, long)]
        name: String,
    },
    /// 发布内容
    PostContent {
        #[arg(short, long)]
        content: String,
    },
    /// 获取用户信息
    GetUser {
        #[arg(short, long)]
        address: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let rpc_client = RpcClient::new(cli.url.clone());

    println!("连接到Solana网络: {}", cli.url);

    // 创建运行时
    let runtime = tokio::runtime::Runtime::new()?;

    match cli.command {
        Commands::CreateUser { name } => {
            println!("创建用户: {}", name);
            
            // 加载或生成密钥对
            let keypair = get_or_create_keypair()?;
            println!("使用公钥: {}", keypair.pubkey());
            
            // 执行创建用户操作
            runtime.block_on(utils::create_user(&rpc_client, &keypair, name))?;
        }
        Commands::PostContent { content } => {
            println!("发布内容: {}", content);
            
            // 加载或生成密钥对
            let keypair = get_or_create_keypair()?;
            println!("使用公钥: {}", keypair.pubkey());
            
            // 执行发布内容操作
            runtime.block_on(utils::post_content(&rpc_client, &keypair, content))?;
        }
        Commands::GetUser { address } => {
            println!("获取用户信息: {}", address);
            
            // 执行获取用户信息操作
            match runtime.block_on(utils::get_user(&rpc_client, &address))? {
                Some(user) => {
                    println!("\n用户信息:");
                    println!("公钥: {}", user.pubkey);
                    println!("用户名: {}", user.name);
                    println!("简介: {}", user.bio);
                    println!("关注者: {}", user.followers_count);
                    println!("关注数: {}", user.following_count);
                    println!("创建时间: {}", user.created_at);
                    println!("更新时间: {}", user.updated_at);
                },
                None => println!("未找到用户信息"),
            }
        }
    }

    Ok(())
}

/// 获取或创建密钥对
fn get_or_create_keypair() -> Result<Keypair, Box<dyn Error>> {
    // 这里简化处理，直接创建新密钥对
    // 实际应用中应该从文件加载或使用更安全的方式管理
    let keypair = Keypair::new();
    Ok(keypair)
}