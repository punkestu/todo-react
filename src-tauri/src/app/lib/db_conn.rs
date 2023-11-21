use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use tokio::runtime::Runtime;

const DB_URL: &str = "data.db";

pub fn gen_pool(rt: &Runtime) -> Result<Pool<Sqlite>, String> {
    rt.block_on(async {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("Creating database {}", DB_URL);
            match Sqlite::create_database(DB_URL).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(DB_URL)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS todo (id INTEGER PRIMARY KEY AUTOINCREMENT,label VARCHAR(50) NOT NULL,state BIT NOT NULL DEFAULT 0);").execute(&pool).await.unwrap();
        Ok(pool)
    })
}
