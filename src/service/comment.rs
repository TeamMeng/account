use crate::{AppError, AppState, Comment};

impl AppState {
    pub async fn create_comment(
        &self,
        pid: i64,
        uid: i64,
        content: &str,
    ) -> Result<Comment, AppError> {
        let comment = sqlx::query_as(
            "
            insert into wb_comment (pid, uid, content) values ($1, $2, $3) returning *
            ",
        )
        .bind(pid)
        .bind(uid)
        .bind(content)
        .fetch_one(&self.pool)
        .await?;

        Ok(comment)
    }

    pub async fn delete_comment(&self, pid: i64, uid: i64) -> Result<(), AppError> {
        if self.find_post_by_pid(pid).await?.is_none() {
            return Err(AppError::NotFound(format!("Post by {} not found", pid)));
        }

        sqlx::query(
            "
            delete from wb_comment where pid = $1 and uid = $2
            ",
        )
        .bind(pid)
        .bind(uid)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_comments(&self, pid: i64) -> Result<Vec<Comment>, AppError> {
        if self.find_post_by_pid(pid).await?.is_none() {
            return Err(AppError::NotFound(format!("Post by {} not found", pid)));
        }

        let comments = sqlx::query_as(
            "
            select * from wb_comment where pid = $1
            ",
        )
        .bind(pid)
        .fetch_all(&self.pool)
        .await?;

        Ok(comments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CreatePost;
    use anyhow::Result;

    #[tokio::test]
    async fn create_delete_comment_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let phone = "19876543210";

        let user = state
            .find_user_by_phone(phone)
            .await?
            .expect("User should exist");

        let post = CreatePost::new("Hello World");
        let post = state.create_post(user.clone(), &post.content).await?;

        let content = "Hello!";
        let comment = state.create_comment(post.pid, user.uid, content).await?;

        assert_eq!(comment.pid, post.pid);
        assert_eq!(comment.uid, user.uid);
        assert_eq!(comment.content, content);

        let comments = state.get_all_comments(post.pid).await?;

        assert_eq!(comments.len(), 1);

        state.delete_comment(post.pid, user.uid).await?;

        Ok(())
    }
}
