/// This function is used to avoid [rustc(E0282)](https://doc.rust-lang.org/error_codes/E0282.html) error in `#[validate(custom = ...)]` validator on the struct.
#[inline]
pub fn wrap_closure_validation<T: ?Sized, M: IntoVecErrors>(
    data: &T,
    f: impl FnOnce(&T) -> Result<(), M>,
) -> Result<(), Vec<crate::validation::Error>> {
    f(data).map_err(|e| e.into_vec_errors())
}

#[inline]
pub fn wrap_into_vec_errors<M: IntoVecErrors>(
    result: Result<(), M>,
) -> Result<(), Vec<crate::validation::Error>> {
    result.map_err(|e| e.into_vec_errors())
}

pub trait IntoVecErrors {
    fn into_vec_errors(self) -> Vec<crate::validation::Error>;
}

impl IntoVecErrors for Vec<crate::validation::Error> {
    fn into_vec_errors(self) -> Vec<crate::validation::Error> {
        self
    }
}

impl IntoVecErrors for crate::validation::Error {
    fn into_vec_errors(self) -> Vec<crate::validation::Error> {
        vec![self]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_custom_fn_single_error() {
        fn single_error(data: &i32) -> Result<(), crate::validation::Error> {
            if *data > 0 {
                Ok(())
            } else {
                Err(crate::validation::Error::Custom(
                    "Value must be greater than 0".to_string(),
                ))
            }
        }

        assert!(wrap_closure_validation(&1i32, single_error).is_ok());
    }

    #[test]
    fn test_custom_fn_multiple_errors() {
        fn multiple_errors(data: &i32) -> Result<(), Vec<crate::validation::Error>> {
            let mut errors = Vec::new();
            if *data > 0 {
                return Ok(());
            } else {
                errors.push(crate::validation::Error::Custom(
                    "Value must be greater than 0".to_string(),
                ));
            }

            if *data < 10 {
                return Ok(());
            } else {
                errors.push(crate::validation::Error::Custom(
                    "Value must be less than 10".to_string(),
                ));
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors)
            }
        }

        assert!(wrap_closure_validation(&1i32, multiple_errors).is_ok());
    }
}
