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
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn build_info(&self) -> async_graphql::Result<crate::app::model::BuildInfo> {
        crate::app::model::BuildInfo::collect().map_err(Into::into)
    }
}

#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn gui<'ctx>(&self, ctx: &Context<'ctx>, data: serde_json::Value) -> async_graphql::Result<bool> {
        let this = serde_json::from_value(data)?;
        let model = ctx.data::<crate::app::Model>()?;
        model.update_gui(|that| *that = this).await?;
        Ok(true)
    }
}

#[derive(Default)]
pub struct Subscription;

#[Subscription]
impl Subscription {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    async fn gui<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> async_graphql::Result<impl Stream<Item = async_graphql::Result<serde_json::Value>> + 'ctx> {
        let model = ctx.data::<crate::app::Model>()?;
        let stream = async_stream::try_stream! {
            yield serde_json::to_value(model.gui.read().await.clone())?;
            loop {
                model.notifiers.gui.notified().await;
                yield serde_json::to_value(model.gui.read().await.clone())?;
            }
        };
        Ok(stream)
    }
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn schema(state: crate::app::Model) -> Schema<Query, Mutation, Subscription> {
    let query = Query::default();
    let mutation = Mutation::default();
    let subscription = Subscription::default();
    Schema::build(query, mutation, subscription).data(state).finish()
}
