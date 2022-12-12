mod schema;

pub use schema::schema;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Provenience {
    #[default]
    Backend,
    Frontend,
}

#[derive(Clone, Debug)]
pub struct Payload {
    pub provenience: Provenience,
    pub config: crate::app::model::Config,
}
