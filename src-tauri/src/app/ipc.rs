mod schema;

pub use schema::schema;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Provenience {
    #[default]
    Backend,
    Frontend,
}

#[derive(Debug, Default)]
pub struct Payload {
    pub provenience: Provenience,
    pub config: crate::app::model::Config,
}
