use crate::models::errors::DatabaseError;
use coi::{Inject, Provide};
use error_stack::{Report, ResultExt};
use sqlx::PgPool;
use std::env;

pub async fn crete_database_connection() -> Result<PgPool, Report<DatabaseError>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .attach_printable_lazy(|| "Failed to connect to database")
        .change_context(DatabaseError)?;

    Ok(pool)
}

#[derive(Inject)]
pub struct PgPoolWrapper(PgPool);

impl PgPoolWrapper {
    pub fn get(&self) -> PgPool {
        self.0.clone()
    }
}

#[derive(Provide)]
#[coi(provides PgPoolWrapper with PgPoolWrapper(self.0.clone()))]
pub struct PgPoolProvider(PgPool);

impl PgPoolProvider {
    pub fn new(db: PgPool) -> Self {
        Self(db)
    }
}
