pub use creator::*;
pub use crud::ManagedQuiz;
pub use fragments::*;
pub use full::*;
pub use queries::*;
pub use quiz::*;
pub use rounds::*;

mod creator;
mod crud;
mod fragments;
mod full;
mod queries;
mod quiz;
mod rounds;

mod schema {
    cynic::use_schema!("schema.gql");
}
