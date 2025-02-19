use crate::{AppError, AppState, Like};

impl AppState {
    pub async fn create_like(&self, uid: i64, pid: i64) -> Result<(), AppError> {
        if self.find_post_by_pid(pid).await?.is_none() {
            return Err(AppError::NotFound(format!("Post by {} not found", pid)));
        }

        sqlx::query(
            "
            insert into wb_like (pid, uid) values ($1, $2)
            ",
        )
        .bind(pid)
        .bind(uid)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_like(&self, uid: i64, pid: i64) -> Result<(), AppError> {
        if self.find_post_by_pid(pid).await?.is_none() {
            return Err(AppError::NotFound(format!("Post by {} not found", pid)));
        }

        sqlx::query(
            "
            delete from wb_like where pid = $1 and uid = $2
            ",
        )
        .bind(pid)
        .bind(uid)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_likes_num(&self, pid: i64) -> Result<i64, AppError> {
        if self.find_post_by_pid(pid).await?.is_none() {
            return Err(AppError::NotFound(format!("Post by {} not found", pid)));
        }

        let likes: Vec<Like> = sqlx::query_as(
            "
            select * from wb_like where pid = $1
            ",
        )
        .bind(pid)
        .fetch_all(&self.pool)
        .await?;

        Ok(likes.len() as _)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CreatePost;
    use anyhow::Result;

    #[tokio::test]
    async fn create_delete_get_like_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let phone = "19876543210";

        let user = state
            .find_user_by_phone(phone)
            .await?
            .expect("User should exist");

        let post = CreatePost::new("Hello World");
        let post = state.create_post(user.clone(), &post.content).await?;

        state.create_like(user.uid, post.pid).await?;

        let num = state.get_likes_num(post.pid).await?;

        assert_eq!(num, 1);

        state.delete_feed(user.uid, post.pid).await?;

        Ok(())
    }
}
