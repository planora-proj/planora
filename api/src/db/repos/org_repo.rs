use sea_query::*;
use sqlx::PgPool;

use crate::db::{DBResult, dto::organization::CreateOrg, models::Organization};

const PG_TABLE_ORGS: &'static str = "organizations";

pub struct OrgRepo<'a> {
    pub pool: &'a PgPool,
}

impl<'a> OrgRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_org(
        &self,
        org: &CreateOrg,
        owner_id: uuid::Uuid,
    ) -> DBResult<Organization> {
        let query = Query::insert()
            .into_table(Alias::new(PG_TABLE_ORGS))
            .columns(["owner_id", "name", "subdomain"])
            .values_panic([
                owner_id.into(),
                org.name.to_owned().into(),
                org.subdomain.to_owned().into(),
            ])
            .returning_all()
            .to_string(PostgresQueryBuilder);

        let inserted_org = sqlx::query_as::<_, Organization>(&query)
            .fetch_one(self.pool)
            .await?;

        Ok(inserted_org)
    }

    pub async fn find_by_ownerid(&self, owner_id: uuid::Uuid) -> DBResult<Vec<Organization>> {
        let query = Query::select()
            .column(Asterisk)
            .from(PG_TABLE_ORGS)
            .and_where(Expr::col(Alias::new("owner_id")).eq(owner_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let org = sqlx::query_as::<_, Organization>(&query)
            .fetch_all(self.pool)
            .await?;
        Ok(org)
    }

    pub async fn find_by_orgid(&self, org_id: uuid::Uuid) -> DBResult<Option<Organization>> {
        let query = Query::select()
            .column(Asterisk)
            .from(PG_TABLE_ORGS)
            .and_where(Expr::col(Alias::new("organization_id")).eq(org_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let org = sqlx::query_as::<_, Organization>(&query)
            .fetch_optional(self.pool)
            .await?;
        Ok(org)
    }

    pub async fn delete_by_orgid(&self, org_id: uuid::Uuid) -> DBResult<u64> {
        let query = Query::delete()
            .from_table(Alias::new(PG_TABLE_ORGS))
            .and_where(Expr::col(Alias::new("organization_id")).eq(org_id.to_string()))
            .to_string(PostgresQueryBuilder);

        let result = sqlx::query(&query).execute(self.pool).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_by_subdomain(&self, subdomain: String) -> DBResult<u64> {
        let query = Query::delete()
            .from_table(Alias::new(PG_TABLE_ORGS))
            .and_where(Expr::col(Alias::new("subdomain")).eq(subdomain))
            .to_string(PostgresQueryBuilder);

        let result = sqlx::query(&query).execute(self.pool).await?;
        Ok(result.rows_affected())
    }
}
