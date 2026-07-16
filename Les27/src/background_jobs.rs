use sqlx::{PgPool, query};

pub async fn start_expired_code_cleanup(pool: PgPool){
    tokio::spawn(async move{
        let mut interval = tokio::time::interval(
            std::time::Duration::from_secs(86400)
        );

        loop{
            interval.tick().await;

            match delete_expired_codes(&pool).await{
                Ok(v) => {
                    tracing::info!("Cleanup: {} expired codes deleted.", v);
                },
                Err(e) => {
                    tracing::error!("Cleanup failed: {e}");
                }
            }
        }
    });
}

async fn delete_expired_codes(pool: &PgPool) -> Result<u64, sqlx::Error>{
    let result = query!(r#"
        DELETE
        FROM user_codes
        WHERE expires_at < NOW()
    "#)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}