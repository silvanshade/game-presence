mod schema;

pub use schema::schema;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Provenience {
    Backend,
    Frontend,
}

#[derive(Debug)]
pub struct Payload<T> {
    pub provenience: Provenience,
    pub data: T,
}

impl<T> Payload<T> {
    pub fn from_backend(data: T) -> Self {
        let provenience = Provenience::Backend;
        Self { provenience, data }
    }

    pub fn from_frontend(data: T) -> Self {
        let provenience = Provenience::Frontend;
        Self { provenience, data }
    }

    pub fn is_from_backend(&self) -> bool {
        self.provenience == Provenience::Backend
    }

    pub fn is_from_frontend(&self) -> bool {
        self.provenience == Provenience::Frontend
    }
}
