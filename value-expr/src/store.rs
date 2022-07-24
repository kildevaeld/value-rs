use async_trait::async_trait;
use value::Value;

use crate::Expr;

pub trait Query<V>: Send + Sync {
    fn filter(&self) -> Expr<String, V>;
}

#[async_trait]
pub trait Store {
    type Value: Send;
    type Create;
    type Update;
    type Filter: Query<Self::Value>;
    type Error;

    async fn fetch(&self, query: &Self::Filter) -> Result<Vec<Self::Value>, Self::Error>;
    async fn fetch_one(&self, query: &Self::Filter) -> Result<Self::Value, Self::Error>;
    async fn fetch_count(
        &self,
        query: &Self::Filter,
    ) -> Result<(usize, Vec<Self::Value>), Self::Error> {
        let result = self.fetch(query).await?;
        let count = self.count(query).await?;

        Ok((count, result))
    }

    async fn count(&self, query: &Self::Filter) -> Result<usize, Self::Error>;

    async fn create(&self, create: Self::Create) -> Result<Value, Self::Error>;
    async fn update(&self, update: Self::Update) -> Result<(), Self::Error>;

    async fn remove(&self, query: Self::Filter) -> Result<(), Self::Error>;
}
