use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {}

pub async fn run(state: crate::app::model::Model) -> Result<(), Error> {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        // println!("loop");
    }
}
