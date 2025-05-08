use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use log::{info, warn, error};
use dotenv::dotenv;

pub async fn init_db() -> Option<PgPool> {
    // Load environmental variables from .env
    dotenv().ok();
    
    let db_url: String = get_db_url().await; 
    info!("Connecting to DB: {}.", db_url);

    let max_cn = 5;

    match PgPoolOptions::new()
        .max_connections(max_cn)
        .connect(&db_url)
        .await
    {
        Ok(pool) => Some(pool),
        Err(e) => {
            error!("The DB connection failed: {}", e); // 👈 THIS IS CRUCIAL
            None
        }
    }
}


async fn get_db_url() -> (String) {
    // This closure is needed to reduce redundant complexity within the code 
    // Returns either on success the desired value OR the default one    
    let get_env_var = |key : &str, alter : &str| {
        env::var(key).unwrap_or_else(|e: env::VarError| alter.to_string())
    };

    // Getting the credentials by using the closure
    let db_user= get_env_var("DB_USER", "postgres");
    let password= get_env_var("DB_PASSWORD", "");
    let host= get_env_var("DB_HOST", "localhost");
    let port= get_env_var("DB_PORT", "5432");
    let db_name= get_env_var("DB_NAME", "database_name");

    // Createing connection url to database
    let connection_url = if password.is_empty() {
        format!("postgresql://{}@{}:{}/{}?sslmode=require", db_user, host, port, db_name)
    } else {
        format!(
            "postgresql://{}:{}@{}:{}/{}?sslmode=require",
            db_user, password, host, port, db_name
        )
    };

    connection_url
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    

    #[tokio::test]
    async fn test_db_connection() {
        let pool = init_db().await;
        // Load environmental variables from .env
        dotenv().ok();

        assert!(pool.is_some(), "DB TEST CONNECTION PASSED");

        if let Some(pool) = pool {
            let result = sqlx::query("SELECT 1")
            .fetch_one(&pool)
            .await;

        assert!(result.is_ok(), "DB TEST QUERY SUCCEED");
        }
    }
}