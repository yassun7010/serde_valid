mod array;
mod error;
mod generic;
mod numeric;
mod object;
mod string;

pub use array::{ValidateMaxItems, ValidateMinItems, ValidateUniqueItems};
pub use error::{
    ArrayErrors, Composited, Error, Errors, IntoError, MapErrors, ObjectErrors, VecErrors,
};
pub use generic::{Literal, ValidateCompositedEnumerate, ValidateEnumerate};
pub use numeric::{
    Number, ValidateCompositedExclusiveMaximum, ValidateCompositedExclusiveMinimum,
    ValidateCompositedMaximum, ValidateCompositedMinimum, ValidateCompositedMultipleOf,
    ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaximum, ValidateMinimum,
    ValidateMultipleOf,
};
pub use object::{ValidateMaxProperties, ValidateMinProperties};
pub use string::{Pattern, ValidateMaxLength, ValidateMinLength, ValidatePattern};

use crate::{
    MaxLengthErrorParams, MaxPropertiesErrorParams, MinLengthErrorParams, MinPropertiesErrorParams,
    PatternErrorParams,
};

macro_rules! impl_composited_validation1 {
    (
        pub trait $ValidateCompositedTrait:ident {
            fn $validate_composited_method:ident(
                &self,
                $limit:ident: $limit_type:ty,
            ) -> Result<(), Composited<$ErrorParams:ty>>;
        }
    ) => {
        paste::paste! {
            pub trait $ValidateCompositedTrait {
                fn $validate_composited_method(
                    &self,
                    $limit: $limit_type,
                ) -> Result<(), Composited<$ErrorParams>>;
            }

            impl<T> $ValidateCompositedTrait for T
            where
                T: [<Validate $limit:camel>],
            {
                fn $validate_composited_method(
                    &self,
                    $limit: $limit_type,
                ) -> Result<(), Composited<$ErrorParams>> {
                    self.[<validate_ $limit>]($limit)
                        .map_err(|error| Composited::Single(error))
                }
            }

            impl<T> $ValidateCompositedTrait for Vec<T>
            where
                T: $ValidateCompositedTrait,
            {
                fn $validate_composited_method(
                    &self,
                    $limit: $limit_type,
                ) -> Result<(), Composited<$ErrorParams>> {
                    let mut errors = vec![];
                    self.iter().for_each(|item| {
                        item.$validate_composited_method($limit)
                            .map_err(|error| errors.push(error))
                            .ok();
                    });

                    if errors.is_empty() {
                        Ok(())
                    } else {
                        Err(Composited::Array(errors))
                    }
                }
            }

            impl<T, const N: usize> $ValidateCompositedTrait for [T; N]
            where
                T: $ValidateCompositedTrait,
            {
                fn $validate_composited_method(
                    &self,
                    $limit: $limit_type,
                ) -> Result<(), Composited<$ErrorParams>> {
                    let mut errors = vec![];
                    self.iter().for_each(|item| {
                        item.$validate_composited_method($limit)
                            .map_err(|error| errors.push(error))
                            .ok();
                    });

                    if errors.is_empty() {
                        Ok(())
                    } else {
                        Err(Composited::Array(errors))
                    }
                }
            }

            impl<T> $ValidateCompositedTrait for Option<T>
            where
                T: $ValidateCompositedTrait,
            {
                fn $validate_composited_method(
                    &self,
                    $limit: $limit_type,
                ) -> Result<(), Composited<$ErrorParams>> {
                    match self {
                        Some(value) => value.$validate_composited_method($limit),
                        None => Ok(()),
                    }
                }
            }
        }
    };
}

// String
impl_composited_validation1!(
    pub trait ValidateCompositedMaxLength {
        fn validate_composited_max_length(
            &self,
            max_length: usize,
        ) -> Result<(), Composited<MaxLengthErrorParams>>;
    }
);

impl_composited_validation1!(
    pub trait ValidateCompositedMinLength {
        fn validate_composited_min_length(
            &self,
            min_length: usize,
        ) -> Result<(), Composited<MinLengthErrorParams>>;
    }
);

impl_composited_validation1!(
    pub trait ValidateCompositedPattern {
        fn validate_composited_pattern(
            &self,
            pattern: &regex::Regex,
        ) -> Result<(), Composited<PatternErrorParams>>;
    }
);

// Object
impl_composited_validation1!(
    pub trait ValidateCompositedMaxProperties {
        fn validate_composited_max_properties(
            &self,
            max_properties: usize,
        ) -> Result<(), Composited<MaxPropertiesErrorParams>>;
    }
);

impl_composited_validation1!(
    pub trait ValidateCompositedMinProperties {
        fn validate_composited_min_properties(
            &self,
            min_properties: usize,
        ) -> Result<(), Composited<MinPropertiesErrorParams>>;
    }
);
