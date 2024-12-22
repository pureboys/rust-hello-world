mod router;
mod service;
mod config;
mod entity;

use clap::Parser;
use tracing::{debug, Level};
use tracing_subscriber;
use anyhow::Result;
use sea_orm::DatabaseConnection;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'c', long = "config", default_value = "config/config.toml")]
    config: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
}


#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();

    let configs = get_configs();
    let configs = config::configs::parse_toml(configs)?;

    // 导入mysql配置
    let mysql: DatabaseConnection = config::mysql::connect_mysql(configs).await?;
    let app_state = AppState { db: mysql };

    debug!("Starting server...");
    let router = router::routers::router(app_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, router).await?;

    Ok(())
}

// 获取配置文件
fn get_configs() -> String {
    let args = Args::parse();
    args.config
}


