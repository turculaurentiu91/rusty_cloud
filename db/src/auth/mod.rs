use sqlx::PgPool;

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub enum LoginErrors {
    UserNotFound,
    DatabaseError(sqlx::Error),
}

pub enum RegisterErrors {
    EmailTaken,
    DatabaseError(sqlx::Error),
}

impl User {
    pub async fn login(email: &str, password: &str, db: &PgPool) -> Result<User, LoginErrors> {
        let result = sqlx::query!(
            r#"
                SELECT
                    id,
                    name,
                    email
                FROM
                    users
                WHERE
                    email = $1
                    AND password = crypt($2, password)
            "#,
            email,
            password
        )
        .fetch_optional(db)
        .await;

        match result {
            Err(e) => Err(LoginErrors::DatabaseError(e)),
            Ok(None) => Err(LoginErrors::UserNotFound),
            Ok(Some(user)) => Ok(User {
                id: user.id,
                name: user.name,
                email: user.email,
            }),
        }
    }

    pub async fn register(
        name: &str,
        email: &str,
        password: &str,
        db: &PgPool,
    ) -> Result<User, RegisterErrors> {
        let result = sqlx::query!(
            r#"
                INSERT INTO users (name, email, password)
                VALUES ($1, $2, crypt($3, gen_salt('bf')))
                RETURNING id, name, email
            "#,
            name,
            email,
            password
        )
        .fetch_one(db)
        .await;

        match result {
            Err(e) => {
                if e.as_database_error()
                    .is_some_and(|e| e.code().map(|c| c == "23505").unwrap_or(false))
                {
                    Err(RegisterErrors::EmailTaken)
                } else {
                    Err(RegisterErrors::DatabaseError(e))
                }
            }
            Ok(user) => Ok(User {
                id: user.id,
                name: user.name,
                email: user.email,
            }),
        }
    }
}
