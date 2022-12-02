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
    async fn state<'ctx>(&self, ctx: &Context<'ctx>, data: serde_json::Value) -> async_graphql::Result<bool> {
        let data = serde_json::from_value(data)?;
        let config = crate::app::ipc::Payload::from_frontend(data);
        let state = ctx.data::<crate::app::model::State>()?;
        state.config.tx.lock().await.send(config)?;
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
                let mut rx = state.config.rx.write().await;
                rx.changed().await?;
                if rx.borrow().is_from_backend() {
                    let data = rx.borrow().data.clone();
                    yield serde_json::to_value(data)?;
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
