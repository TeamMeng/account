use crate::{AppError, AppState};

impl AppState {
    pub async fn create_feed(&self, pid: i64, uid: i64) -> Result<(), AppError> {
        sqlx::query(
            "
            insert into wb_feed (pid, uid) values ($1, $2)
            ",
        )
        .bind(pid)
        .bind(uid)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_feed(&self, pid: i64, uid: i64) -> Result<(), AppError> {
        sqlx::query(
            "
            delete from wb_feed where pid = $1 and uid = $2
            ",
        )
        .bind(pid)
        .bind(uid)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn create_and_delete_feed_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        state.create_feed(2, 1).await?;
        state.delete_feed(2, 1).await?;

        Ok(())
    }
}
