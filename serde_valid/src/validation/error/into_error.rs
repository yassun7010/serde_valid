pub trait IntoError<E>: Sized {
    fn into_error(self) -> crate::validation::Error {
        self.into_error_by(crate::validation::error::Format::Default)
    }

    fn into_error_by(self, format: crate::validation::error::Format<E>)
        -> crate::validation::Error;
}
