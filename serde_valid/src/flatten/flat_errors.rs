use super::FlatError;

#[derive(Debug, PartialEq, Eq)]
pub struct FlatErrors(Vec<FlatError>);

impl FlatErrors {
    pub fn new(errors: impl Into<Vec<FlatError>>) -> Self {
        Self(errors.into())
    }
}

impl IntoIterator for FlatErrors {
    type Item = FlatError;
    type IntoIter = <Vec<FlatError> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a FlatErrors {
    type Item = &'a FlatError;
    type IntoIter = std::slice::Iter<'a, FlatError>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl From<Vec<FlatError>> for FlatErrors {
    fn from(errors: Vec<FlatError>) -> Self {
        Self(errors)
    }
}
