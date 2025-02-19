use crate::{AppError, AppState, Post, User};

impl AppState {
    // 创建文章
    pub async fn create_post(&self, user: User, content: &str) -> Result<Post, AppError> {
        let post: Post = sqlx::query_as(
            "
            insert into wb_post (uid, content) values ($1, $2) returning *
            ",
        )
        .bind(user.uid)
        .bind(content)
        .fetch_one(&self.pool)
        .await?;

        let uids = self.get_all_follower(user).await?;

        for uid in uids {
            self.create_feed(post.pid, uid).await?;
        }

        Ok(post)
    }

    // 删除文章
    pub async fn delete_post(&self, user: User, pid: i64) -> Result<(), AppError> {
        if self.find_post(pid, user.uid).await?.is_none() {
            return Err(AppError::NotFound(format!("Post by {} not found", pid)));
        }

        let uids = self.get_all_followee(user.clone()).await?;

        for uid in uids {
            self.delete_feed(pid, uid).await?;
        }

        sqlx::query(
            "
            update wb_post set is_deleted = true where uid = $1 and pid = $2
            ",
        )
        .bind(user.uid)
        .bind(pid)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_post(&self, pid: i64, uid: i64) -> Result<Option<Post>, AppError> {
        let post = sqlx::query_as(
            "
            select * from wb_post where uid = $1 and pid = $2 and is_deleted = false
            ",
        )
        .bind(uid)
        .bind(pid)
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }

    // 获得全部文章
    pub async fn get_all_posts(&self, uid: i64) -> Result<Vec<Post>, AppError> {
        let posts = sqlx::query_as(
            "
            select * from wb_post where uid = $1 and is_deleted = false
            ",
        )
        .bind(uid)
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    pub async fn find_post_by_pid(&self, pid: i64) -> Result<Option<Post>, AppError> {
        let post = sqlx::query_as(
            "
            select * from wb_post where pid = $1 and is_deleted = false
            ",
        )
        .bind(pid)
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn create_delete_find_get_posts_and_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let phone = "19876543210";

        let user = state
            .find_user_by_phone(phone)
            .await?
            .expect("User should exist");

        let content = "Hello World";

        let post = state.create_post(user.clone(), content).await?;

        let ret = state.find_post(post.pid, user.uid).await?;
        assert!(ret.is_some());

        let ret = state.find_post_by_pid(post.pid).await?;
        assert!(ret.is_some());

        assert_eq!(post.uid, user.uid);
        assert_eq!(post.content, content);

        let vec = state.get_all_posts(user.uid).await?;
        assert_eq!(vec.len(), 1);

        state.delete_post(user.clone(), vec[0].pid).await?;

        let vec = state.get_all_posts(user.uid).await?;
        assert_eq!(vec.len(), 0);

        Ok(())
    }
}
