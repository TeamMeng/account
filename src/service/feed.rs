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
            delete from wb_feed where pid = $1 AND uid = $2;
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
    use crate::CreatePost;
    use anyhow::Result;

    #[tokio::test]
    async fn create_and_delete_feed_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let phone = "19876543210";

        let user = state
            .find_user_by_phone(phone)
            .await?
            .expect("User should exist");

        let post = CreatePost::new("Hello World");
        let post = state.create_post(user.clone(), &post.content).await?;

        state.create_feed(post.pid, user.uid).await?;
        state.delete_feed(post.pid, user.uid).await?;

        Ok(())
    }
}
