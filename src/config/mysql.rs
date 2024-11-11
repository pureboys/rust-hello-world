use sea_orm::{Database, DatabaseConnection};
use crate::config::configs::Config;

pub async fn connect_mysql(config: Config) -> Result<DatabaseConnection, anyhow::Error> {
    let protocol = format!("mysql://{}:{}@{}:{}/{}",
                           config.mysql.user,
                           config.mysql.passwd,
                           config.mysql.host,
                           config.mysql.port,
                           config.mysql.db);
    let db: DatabaseConnection = Database::connect(protocol).await?;
    Ok(db)
}
