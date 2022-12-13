use async_graphql::{futures_util::*, *};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    SerdeJsonSerialize { source: serde_json::Error },
}

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn build_info(&self) -> async_graphql::Result<crate::app::model::BuildInfo> {
        crate::app::model::BuildInfo::collect().map_err(Into::into)
    }
}

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    async fn state<'ctx>(&self, ctx: &Context<'ctx>, data: serde_json::Value) -> async_graphql::Result<bool> {
        let config = serde_json::from_value(data)?;
        let state = ctx.data::<crate::app::model::State>()?;
        state.update_without_rebroadcast(config).await?;
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
                let mut rx = state.rx.write().await;
                rx.changed().await?;
                if rx.borrow().provenience == crate::app::ipc::Provenience::Backend {
                    let config = rx.borrow().config.clone();
                    yield serde_json::to_value(config)?;
                }
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
