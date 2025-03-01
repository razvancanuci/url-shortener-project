use crate::database::pool::PgPoolWrapper;
use crate::models::errors::DatabaseError;
use crate::models::url_models::Url;
use async_trait::async_trait;
use coi::Inject;
use error_stack::{Report, ResultExt};
use mockall::automock;
use std::sync::Arc;

#[async_trait]
#[automock]
pub trait UrlRepositoryTrait: Inject {
    async fn create(&self, url: Url) -> Result<Url, Report<DatabaseError>>;
    async fn find(&self, short_url: &str) -> Result<Option<Url>, Report<DatabaseError>>;
}

#[derive(Inject)]
#[coi(provides pub dyn UrlRepositoryTrait with UrlRepository::new(db))]
pub struct UrlRepository {
    #[coi(inject)]
    pub db: Arc<PgPoolWrapper>,
}

impl UrlRepository {
    pub fn new(db: Arc<PgPoolWrapper>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UrlRepositoryTrait for UrlRepository {
    async fn create(&self, url: Url) -> Result<Url, Report<DatabaseError>> {
        let result = sqlx::query_as::<_, Url>(
            r#"
        INSERT INTO urls (id, url)
        VALUES ($1, $2)
        RETURNING id, url
        "#,
        )
        .bind(&url.id)
        .bind(&url.url)
        .fetch_one(&self.db.get())
        .await
        .attach_printable_lazy(move || format!("Failed to create url: {:?}", url))
        .change_context(DatabaseError)?;

        Ok(result)
    }

    async fn find(&self, short_url: &str) -> Result<Option<Url>, Report<DatabaseError>> {
        let user = sqlx::query_as::<_, Url>("SELECT id, url FROM urls WHERE id = $1")
            .bind(short_url)
            .fetch_optional(&self.db.get())
            .await
            .attach_printable_lazy(|| format!("Failed to find url with id: {}", short_url))
            .change_context(DatabaseError)?;

        Ok(user)
    }
}

// for mocking
impl Inject for MockUrlRepositoryTrait {}
