use crate::{AppError, AppState, CreateFollower, Follower, User};

impl AppState {
    pub async fn create_follower(&self, input: CreateFollower, user: User) -> Result<(), AppError> {
        if is_same(self, input.followee as _, user.uid).await? {
            return Err(AppError::NotFound(format!(
                "Followee's {} is same as user id",
                input.followee
            )));
        }

        sqlx::query(
            "
                insert into wb_follower (follower_id, followee_id)
                values ($1, $2)
                on conflict (follower_id, followee_id) do update
                set is_deleted = false;
            ",
        )
        .bind(user.uid)
        .bind(input.followee)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // 查询用户的粉丝列表
    pub async fn get_all_followee(&self, user: User) -> Result<Vec<i64>, AppError> {
        let followees: Vec<Follower> = sqlx::query_as(
            "
            select * from wb_follower where followee_id = $1 and is_deleted = false
            ",
        )
        .bind(user.uid)
        .fetch_all(&self.pool)
        .await?;

        let ans = followees.into_iter().map(|f| f.follower_id).collect();

        Ok(ans)
    }

    // 查询用户的关注列表
    pub async fn get_all_follower(&self, user: User) -> Result<Vec<i64>, AppError> {
        let followers: Vec<Follower> = sqlx::query_as(
            "
            select * from wb_follower where follower_id = $1 and is_deleted = false
            ",
        )
        .bind(user.uid)
        .fetch_all(&self.pool)
        .await?;

        let ans = followers.into_iter().map(|f| f.followee_id).collect();

        Ok(ans)
    }

    pub async fn delete_followee(&self, user: User, id: i64) -> Result<(), AppError> {
        if is_same(self, id, user.uid).await? {
            return Err(AppError::NotFound(format!(
                "Followee's {} is same as user id",
                id
            )));
        }

        sqlx::query(
            "
            update wb_follower set is_deleted = true where follower_id = $1 AND followee_id = $2
            ",
        )
        .bind(user.uid)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

async fn is_same(state: &AppState, pid: i64, uid: i64) -> Result<bool, AppError> {
    if state.find_user_by_uid(pid).await?.is_none() {
        return Err(AppError::NotFound(format!(
            "Followee id by {} not found",
            pid
        )));
    }
    Ok(pid == uid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn create_followee_and_get_followee_follower_and_delete_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let phone = "19876543210";

        let user = state
            .find_user_by_phone(phone)
            .await?
            .expect("User should exist");

        let phone = "19876543211";

        let followee = state
            .find_user_by_phone(phone)
            .await?
            .expect("User should exist");

        let input = CreateFollower::new(followee.uid as i32);

        state.create_follower(input, user.clone()).await?;

        let vec = state.get_all_follower(user.clone()).await?;
        assert_eq!(vec, [2]);

        let vec = state.get_all_followee(followee.clone()).await?;
        assert_eq!(vec, [1]);

        state.delete_followee(user.clone(), followee.uid).await?;

        let vec = state.get_all_follower(user.clone()).await?;
        assert!(vec.is_empty());

        Ok(())
    }
}
