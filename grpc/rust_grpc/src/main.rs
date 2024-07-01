use tokio_util::sync::CancellationToken;
use std::time::Duration;

mod server;
mod cli;

pub mod greeter {
    tonic::include_proto!("greeter");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = CancellationToken::new();
    let cloned_token = token.clone();

    // 在后台运行服务器
    let server_handle = tokio::spawn(async move {
        if let Err(e) = server::run_server(cloned_token).await {
            eprintln!("Server error: {}", e);
        }
    });

    // 等待一小段时间确保服务器已经启动
    tokio::time::sleep(Duration::from_secs(1)).await;

    // 运行客户端
    cli::run_client().await?;

    // 等待 Ctrl+C 信号
    tokio::signal::ctrl_c().await?;
    println!("Received Ctrl+C, shutting down");

    // 取消服务器
    token.cancel();

    // 等待服务器结束
    server_handle.await?;

    Ok(())
}