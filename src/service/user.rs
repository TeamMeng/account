use crate::{hash_password, verify_password, AppError, AppState, CreateUser, SigninUser, User};

impl AppState {
    pub async fn create_user(&self, input: CreateUser) -> Result<User, AppError> {
        match self.find_user_by_phone(&input.phone).await? {
            Some(user) => Err(AppError::PhoneAlreadyExists(user.phone)),
            None => {
                let password_hash = hash_password(&input.password)?;

                let user = sqlx::query_as(
                    "
                    insert into wb_user (nickname, phone, password_hash) values ($1, $2, $3) returning *
                    ",
                )
                .bind(input.nickname)
                .bind(input.phone)
                .bind(password_hash)
                .fetch_one(&self.pool)
                .await?;
                Ok(user)
            }
        }
    }

    pub async fn find_user_by_phone(&self, phone: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as(
            "
            select * from wb_user where phone = $1
            ",
        )
        .bind(phone)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn signin(&self, input: SigninUser) -> Result<User, AppError> {
        if let Some(user) = self.find_user_by_phone(&input.phone).await? {
            if verify_password(&input.password, &user.password_hash)? {
                return Ok(user);
            }
        }
        Err(AppError::LoginError(format!(
            "Phone or password error by {}",
            input.phone
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::verify_password;

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn create_user_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let phone = "12345678901";
        let password = "123456";
        let nickname = "TeamMeng";
        let input = CreateUser::new(phone, password, nickname);

        let user = state.create_user(input.clone()).await?;

        assert_eq!(user.phone, phone);
        assert_eq!(user.nickname, nickname);
        assert!(verify_password(password, &user.password_hash)?);

        let ret = state.create_user(input).await;

        assert!(ret.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn find_user_by_phone_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let phone = "12345678901";
        let password = "123456";
        let nickname = "TeamMeng";

        let ret = state.find_user_by_phone(phone).await?;

        assert!(ret.is_none());

        let input = CreateUser::new(phone, password, nickname);

        let user = state.create_user(input.clone()).await?;

        assert_eq!(user.phone, phone);
        assert_eq!(user.nickname, nickname);
        assert!(verify_password(password, &user.password_hash)?);

        let user = state.find_user_by_phone(phone).await?;

        assert!(user.is_some());
        let user = user.unwrap();

        assert_eq!(user.phone, phone);
        assert_eq!(user.nickname, nickname);
        assert!(verify_password(password, &user.password_hash)?);

        Ok(())
    }

    #[tokio::test]
    async fn signin_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let phone = "12345678901";
        let password = "123456";
        let nickname = "TeamMeng";

        let input = CreateUser::new(phone, password, nickname);
        state.create_user(input).await?;

        let input = SigninUser::new(phone, password);
        let user = state.signin(input).await?;

        assert_eq!(user.phone, phone);
        assert_eq!(user.nickname, nickname);
        assert!(verify_password(password, &user.password_hash)?);

        let input = SigninUser::new(phone, "hunter42");
        let ret = state.signin(input).await;
        assert!(ret.is_err());

        let input = SigninUser::new("09876543211", password);
        let ret = state.signin(input).await;
        assert!(ret.is_err());

        Ok(())
    }
}
