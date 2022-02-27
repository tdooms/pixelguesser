pub trait EmitError<Err> {
    type Ty;
    fn emit(self, callback: &yew::Callback<Err>) -> Option<Self::Ty>
    where
        Self: Sized;
}

impl<T, E> EmitError<E> for Result<T, E> {
    type Ty = T;
    fn emit(self, callback: &yew::Callback<E>) -> Option<T> {
        match self {
            Ok(t) => Some(t),
            Err(err) => {
                callback.emit(err);
                None
            }
        }
    }
}
