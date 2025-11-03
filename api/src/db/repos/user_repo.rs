use sea_query::*;
use sqlx::PgPool;

use crate::db::models::User;

const PG_TABLE_USERS: &'static str = "users";

pub struct UserRepo<'a> {
    pub pool: &'a PgPool,
}

impl<'a> UserRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: &User) -> sqlx::Result<User> {
        let query = Query::insert()
            .into_table(Alias::new(PG_TABLE_USERS))
            .columns([
                Alias::new("user_tag"),
                Alias::new("username"),
                Alias::new("email"),
                Alias::new("password"),
                Alias::new("timezone"),
                Alias::new("avatar_url"),
                Alias::new("google_sub"),
            ])
            .values_panic([
                user.user_tag.clone().into(),
                user.username.clone().into(),
                user.email.clone().into(),
                user.password.clone().into(),
                user.timezone.clone().into(),
                user.avatar_url.clone().into(),
                user.google_sub.clone().into(),
            ])
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let user = sqlx::query_as::<_, User>(&query)
            .fetch_one(self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: String) -> sqlx::Result<Option<User>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Alias::new(PG_TABLE_USERS))
            .and_where(Expr::col(Alias::new("email")).eq(email))
            .to_string(PostgresQueryBuilder);

        let user = sqlx::query_as::<_, User>(&query)
            .fetch_optional(self.pool)
            .await?;
        Ok(user)
    }

    pub async fn find_by_userid(&self, userid: uuid::Uuid) -> sqlx::Result<Option<User>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Alias::new(PG_TABLE_USERS))
            .and_where(Expr::col(Alias::new("user_id")).eq(userid.to_string()))
            .to_string(PostgresQueryBuilder);

        let user = sqlx::query_as::<_, User>(&query)
            .fetch_optional(self.pool)
            .await?;
        Ok(user)
    }

    pub async fn find_by_usertag(&self, usertag: String) -> sqlx::Result<Option<User>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Alias::new(PG_TABLE_USERS))
            .and_where(Expr::col(Alias::new("user_tag")).eq(usertag))
            .to_string(PostgresQueryBuilder);

        let user = sqlx::query_as::<_, User>(&query)
            .fetch_optional(self.pool)
            .await?;
        Ok(user)
    }

    pub async fn delete_by_email(&self, email: String) -> sqlx::Result<u64> {
        let query = Query::delete()
            .from_table(Alias::new(PG_TABLE_USERS))
            .and_where(Expr::col(Alias::new("email")).eq(email))
            .to_string(PostgresQueryBuilder);

        let result = sqlx::query(&query).execute(self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn list_users(&self, limit: u64, offset: u64) -> sqlx::Result<Vec<User>> {
        let query = Query::select()
            .column(Asterisk)
            .from(Alias::new(PG_TABLE_USERS))
            .limit(limit)
            .offset(offset)
            .order_by(Alias::new("created_at"), Order::Desc)
            .to_string(PostgresQueryBuilder);

        let users = sqlx::query_as::<_, User>(&query)
            .fetch_all(self.pool)
            .await?;
        Ok(users)
    }
}
