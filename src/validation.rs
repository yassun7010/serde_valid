mod array;
mod error;
mod generic;
mod numeric;
mod object;
mod string;

use crate::{
    EnumerateErrorParams, ExclusiveMaximumErrorParams, ExclusiveMinimumErrorParams,
    MaxLengthErrorParams, MaxPropertiesErrorParams, MaximumErrorParams, MinLengthErrorParams,
    MinPropertiesErrorParams, MinimumErrorParams, MultipleOfErrorParams, PatternErrorParams,
};
pub use array::{ValidateMaxItems, ValidateMinItems, ValidateUniqueItems};
pub use error::{
    ArrayErrors, Composited, Error, Errors, IntoError, MapErrors, ObjectErrors, VecErrors,
};
pub use generic::ValidateEnumerate;
pub use numeric::{
    ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaximum, ValidateMinimum,
    ValidateMultipleOf,
};
pub use object::{ValidateMaxProperties, ValidateMinProperties};
pub use serde_valid_literal::{Literal, Number, Pattern};
pub use string::{ValidateMaxLength, ValidateMinLength, ValidatePattern};

macro_rules! impl_composited_validation_1args {
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
    (
        pub trait $ValidateCompositedTrait:ident<T> {
            fn $validate_composited_method:ident(
                &self,
                $limit:ident: T,
            ) -> Result<(), Composited<$ErrorParams:ty>>;
        }
    ) => {
        pub trait $ValidateCompositedTrait<T> {
            fn $validate_composited_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Composited<$ErrorParams>>;
        }

        impl<T, U> $ValidateCompositedTrait<T> for Vec<U>
        where
            T: Copy,
            U: $ValidateCompositedTrait<T>,
        {
            fn $validate_composited_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Composited<$ErrorParams>> {
                let mut errors = vec![];
                self.iter().for_each(|item| {
                    item.$validate_composited_method(limit)
                        .map_err(|error| errors.push(error))
                        .ok();
                });

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(crate::validation::Composited::Array(errors))
                }
            }
        }

        impl<T, U, const N: usize> $ValidateCompositedTrait<T> for [U; N]
        where
            T: Copy,
            U: $ValidateCompositedTrait<T>,
        {
            fn $validate_composited_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Composited<$ErrorParams>> {
                let mut errors = vec![];
                self.iter().for_each(|item| {
                    item.$validate_composited_method(limit)
                        .map_err(|error| errors.push(error))
                        .ok();
                });

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(crate::validation::Composited::Array(errors))
                }
            }
        }

        impl<T, U> $ValidateCompositedTrait<T> for Option<U>
        where
            T: Copy,
            U: $ValidateCompositedTrait<T>,
        {
            fn $validate_composited_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Composited<$ErrorParams>> {
                match self {
                    Some(value) => value.$validate_composited_method(limit),
                    None => Ok(()),
                }
            }
        }
    };
}

macro_rules! impl_generic_composited_validation_1args {
    (
        $ErrorType:ident,
        $type:ty
    ) => {
        paste::paste! {
            impl<T> [<ValidateComposited $ErrorType >]<$type> for T
            where
                T: [<Validate $ErrorType >]<$type>,
            {
                fn [< validate_composited_ $ErrorType:snake>](
                    &self,
                    limit: $type,
                ) -> Result<(), crate::validation::Composited<[<$ErrorType ErrorParams>]>> {
                    self.[< validate_ $ErrorType:snake>](limit)
                        .map_err(|error| crate::validation::Composited::Single(error))
                }
            }
        }
    };
}

pub(crate) use impl_generic_composited_validation_1args;

// Number
impl_composited_validation_1args!(
    pub trait ValidateCompositedMaximum<T> {
        fn validate_composited_maximum(
            &self,
            maximum: T,
        ) -> Result<(), Composited<MaximumErrorParams>>;
    }
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedMinimum<T> {
        fn validate_composited_minimum(
            &self,
            minimum: T,
        ) -> Result<(), Composited<MinimumErrorParams>>;
    }
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedExclusiveMaximum<T> {
        fn validate_composited_exclusive_maximum(
            &self,
            exclusive_maximum: T,
        ) -> Result<(), Composited<ExclusiveMaximumErrorParams>>;
    }
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedExclusiveMinimum<T> {
        fn validate_composited_exclusive_minimum(
            &self,
            exclusive_minimum: T,
        ) -> Result<(), Composited<ExclusiveMinimumErrorParams>>;
    }
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedMultipleOf<T> {
        fn validate_composited_multiple_of(
            &self,
            exclusive_minimum: T,
        ) -> Result<(), Composited<MultipleOfErrorParams>>;
    }
);

// String
impl_composited_validation_1args!(
    pub trait ValidateCompositedMaxLength {
        fn validate_composited_max_length(
            &self,
            max_length: usize,
        ) -> Result<(), Composited<MaxLengthErrorParams>>;
    }
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedMinLength {
        fn validate_composited_min_length(
            &self,
            min_length: usize,
        ) -> Result<(), Composited<MinLengthErrorParams>>;
    }
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedPattern {
        fn validate_composited_pattern(
            &self,
            pattern: &regex::Regex,
        ) -> Result<(), Composited<PatternErrorParams>>;
    }
);

// Object
impl_composited_validation_1args!(
    pub trait ValidateCompositedMaxProperties {
        fn validate_composited_max_properties(
            &self,
            max_properties: usize,
        ) -> Result<(), Composited<MaxPropertiesErrorParams>>;
    }
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedMinProperties {
        fn validate_composited_min_properties(
            &self,
            min_properties: usize,
        ) -> Result<(), Composited<MinPropertiesErrorParams>>;
    }
);

// Generic
impl_composited_validation_1args!(
    pub trait ValidateCompositedEnumerate<T> {
        fn validate_composited_enumerate(
            &self,
            enumerate: T,
        ) -> Result<(), Composited<EnumerateErrorParams>>;
    }
);
