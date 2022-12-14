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
    async fn gui<'ctx>(&self, ctx: &Context<'ctx>, data: serde_json::Value) -> async_graphql::Result<bool> {
        let gui = serde_json::from_value(data)?;
        let model = ctx.data::<crate::app::Model>()?;
        model.write_gui(gui).await?;
        Ok(true)
    }
}

#[derive(Default)]
pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn gui<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> async_graphql::Result<impl Stream<Item = async_graphql::Result<serde_json::Value>> + 'ctx> {
        let model = ctx.data::<crate::app::Model>()?;
        let stream = async_stream::try_stream! {
            let mut initial = true;
            loop {
                if !initial {
                    model.notifiers.gui.notified().await;
                } else {
                    initial = false;
                }
                let gui = model.gui.read().await.clone();
                yield serde_json::to_value(gui)?;
            }
        };
        Ok(stream)
    }
}

pub fn schema(state: crate::app::Model) -> Schema<Query, Mutation, Subscription> {
    let query = Query::default();
    let mutation = Mutation::default();
    let subscription = Subscription::default();
    Schema::build(query, mutation, subscription).data(state).finish()
}
