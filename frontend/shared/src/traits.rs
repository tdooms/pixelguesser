use crate::{Error, Errors};

pub trait EmitError {
    type Ty;
    fn emit(self, error: &Errors) -> Option<Self::Ty>
    where
        Self: Sized;
}

impl<T> EmitError for Result<T, Error> {
    type Ty = T;
    fn emit(self, errors: &Errors) -> Option<T> {
        match self {
            Ok(t) => Some(t),
            Err(err) => {
                errors.emit(err);
                None
            }
        }
    }
}
