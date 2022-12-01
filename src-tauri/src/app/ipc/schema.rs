use async_graphql::{futures_util::*, types::*, *};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    SerdeJsonSerialize { source: serde_json::Error },
}

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn build_info(&self) -> async_graphql::Result<crate::app::data::BuildInfo> {
        crate::app::data::BuildInfo::collect().map_err(Into::into)
    }
}

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    async fn state<'ctx>(&self, ctx: &Context<'ctx>, config: serde_json::Value) -> async_graphql::Result<bool> {
        let config = serde_json::from_value(config)?;
        println!("{:#?}", config);
        let state = ctx.data::<crate::app::model::State>()?;
        let channels = state.config.write().await;
        channels.tx.send(config)?;
        Ok(true)
    }
}

#[derive(Default)]
pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn state<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> async_graphql::Result<impl Stream<Item = async_graphql::Result<serde_json::Value>> + 'ctx> {
        let state = ctx.data::<crate::app::model::State>()?;
        let stream = async_stream::try_stream! {
            loop {
                let mut channels = state.config.write().await;
                channels.rx.changed().await?;
                let config = channels.rx.borrow().clone();
                yield serde_json::to_value(config)?;
            }
        };
        Ok(stream)
    }
}

pub fn schema(state: crate::app::model::State) -> Schema<Query, Mutation, Subscription> {
    let query = Query::default();
    let mutation = Mutation::default();
    let subscription = Subscription::default();
    Schema::build(query, mutation, subscription).data(state).finish()
}
