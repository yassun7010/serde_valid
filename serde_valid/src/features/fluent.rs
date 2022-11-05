use std::collections::HashMap;

use fluent_0::{FluentArgs, FluentBundle, FluentResource};

use crate::{
    error::ToDefaultMessage,
    validation::{ArrayErrors, Errors, ItemErrorsMap, ObjectErrors, PropertyErrorsMap, VecErrors},
};

#[derive(Debug, Clone)]
pub struct FluentError {
    pub id: &'static str,
    pub args: HashMap<&'static str, serde_valid_literal::Literal>,
}

impl std::fmt::Display for FluentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl ToDefaultMessage for FluentError {
    #[inline]
    fn to_default_message(&self) -> String {
        self.id.to_string()
    }
}

trait IntoLocalization {
    type Target;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target;
}

impl IntoLocalization for Errors<crate::validation::Error> {
    type Target = Errors<String>;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        match self {
            Errors::Array(array) => Errors::Array(array.into_localization(bundle)),
            Errors::Object(object) => Errors::Object(object.into_localization(bundle)),
            Errors::NewType(newtype) => Errors::NewType(newtype.into_localization(bundle)),
        }
    }
}

impl IntoLocalization for ArrayErrors<crate::validation::Error> {
    type Target = ArrayErrors<String>;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        ArrayErrors {
            errors: self.errors.into_localization(bundle),
            items: self.items.into_localization(bundle),
        }
    }
}

impl IntoLocalization for ObjectErrors<crate::validation::Error> {
    type Target = ObjectErrors<String>;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        ObjectErrors {
            errors: self.errors.into_localization(bundle),
            properties: self.properties.into_localization(bundle),
        }
    }
}

impl IntoLocalization for VecErrors<crate::validation::Error> {
    type Target = VecErrors<String>;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        self.into_iter()
            .map(|error| error.into_localization(bundle))
            .collect()
    }
}

impl IntoLocalization for ItemErrorsMap<crate::validation::Error> {
    type Target = ItemErrorsMap<String>;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        self.into_iter()
            .map(|(index, error)| (index, error.into_localization(bundle)))
            .collect()
    }
}

impl IntoLocalization for PropertyErrorsMap<crate::validation::Error> {
    type Target = PropertyErrorsMap<String>;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        self.into_iter()
            .map(|(property, error)| (property, error.into_localization(bundle)))
            .collect()
    }
}

impl IntoLocalization for crate::validation::Error {
    type Target = String;

    fn into_localization(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
        match self {
            Self::Fluent(error) => {
                if let Some(message) = bundle.get_message(error.id) {
                    let mut errors = vec![];
                    if let Some(pattern) = message.value() {
                        let args = FluentArgs::new();
                        let value = bundle
                            .format_pattern(pattern, Some(&args), &mut errors)
                            .to_string();
                        if errors.is_empty() {
                            value
                        } else {
                            error.id.to_string()
                        }
                    } else {
                        error.id.to_string()
                    }
                } else {
                    error.id.to_string()
                }
            }
            _ => format!("{self}"),
        }
    }
}
