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
        let res = self.map_err(|x| errors.emit(x));
        res.ok()
    }
}

impl<T> EmitError for Result<T, api::Error> {
    type Ty = T;
    fn emit(self, errors: &Errors) -> Option<T> {
        self.map_err(Error::Api).emit(errors)
    }
}
