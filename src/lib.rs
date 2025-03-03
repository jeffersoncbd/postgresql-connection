use std::env;

use sqlx::{postgres::PgPoolOptions, Connection, PgConnection, Pool, Postgres};
use time;

pub use sqlx::{query, Error};

const FILE_NAME: &str = "💾 PostgreSQL/main";

fn get_database_url() -> String {
    time::log(
        FILE_NAME,
        "Recuperando valor da variável \"DATABASE_URL\"...",
    );
    if let Err(error) = dotenvy::dotenv() {
        let message = format!(
            "{}: \"{}\"",
            "Atenção! erro ao tentar carregar arquivo .env!",
            error.to_string()
        );
        time::log("🟡 PostgreSQL/main", &message);
    }
    env::var("DATABASE_URL")
        .expect("\n\t❌ A variável de ambiente \"DATABASE_URL\" não foi definida!\n\n")
}

pub async fn test_connection() {
    let database_url = get_database_url();
    time::log(
        FILE_NAME,
        "Variável carregada, iniciando teste de conexão com PostgreSQL...",
    );

    match PgConnection::connect(&database_url).await {
        Ok(_) => time::log("✅ PostgreSQL/main", "PostgreSQL conectado!\n"),
        Err(_) => panic!("\n\t❌ Não foi possível estabelecer conexão com PostgreSQL.\n\t{}\n", database_url),
    }
}

pub async fn connect() -> Result<PgConnection, sqlx::Error> {
    let database_url = get_database_url();
    PgConnection::connect(&database_url).await
}

pub async fn open_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = get_database_url();

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
