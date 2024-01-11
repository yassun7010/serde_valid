// use fluent_0::{FluentArgs, FluentBundle, FluentError, FluentResource};

// use crate::validation::error::{
//     ArrayErrors, DefaultFormat, Errors, ItemErrorsMap, ObjectErrors, PropertyErrorsMap, VecErrors,
// };

// pub trait Localize {
//     type Target;

//     fn localize(self, bundle: &FluentBundle<FluentResource>) -> Self::Target;
// }

// impl Localize for Errors<crate::validation::Error> {
//     type Target = Errors<String>;

//     fn localize(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
//         match self {
//             Errors::Array(array) => Errors::Array(array.localize(bundle)),
//             Errors::Object(object) => Errors::Object(object.localize(bundle)),
//             Errors::NewType(newtype) => Errors::NewType(newtype.localize(bundle)),
//         }
//     }
// }

// impl Localize for ArrayErrors<crate::validation::Error> {
//     type Target = ArrayErrors<String>;

//     fn localize(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
//         ArrayErrors {
//             errors: self.errors.localize(bundle),
//             items: self.items.localize(bundle),
//         }
//     }
// }

// impl Localize for ObjectErrors<crate::validation::Error> {
//     type Target = ObjectErrors<String>;

//     fn localize(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
//         ObjectErrors {
//             errors: self.errors.localize(bundle),
//             properties: self.properties.localize(bundle),
//         }
//     }
// }

// impl Localize for VecErrors<crate::validation::Error> {
//     type Target = VecErrors<String>;

//     fn localize(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
//         self.into_iter()
//             .map(|error| error.localize(bundle))
//             .collect()
//     }
// }

// impl Localize for ItemErrorsMap<crate::validation::Error> {
//     type Target = ItemErrorsMap<String>;

//     fn localize(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
//         self.into_iter()
//             .map(|(index, error)| (index, error.localize(bundle)))
//             .collect::<Self::Target>()
//     }
// }

// impl Localize for PropertyErrorsMap<crate::validation::Error> {
//     type Target = PropertyErrorsMap<String>;

//     fn localize(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
//         self.into_iter()
//             .map(|(index, error)| (index, error.localize(bundle)))
//             .collect::<Self::Target>()
//     }
// }

// impl Localize for crate::validation::Error {
//     type Target = String;

//     fn localize(self, bundle: &FluentBundle<FluentResource>) -> Self::Target {
//         match self {
//             Self::Minimum(message) => localize_or_default(&message, bundle),
//             Self::Maximum(message) => localize_or_default(&message, bundle),
//             Self::ExclusiveMinimum(message) => localize_or_default(&message, bundle),
//             Self::ExclusiveMaximum(message) => localize_or_default(&message, bundle),
//             Self::MultipleOf(message) => localize_or_default(&message, bundle),
//             Self::MinLength(message) => localize_or_default(&message, bundle),
//             Self::MaxLength(message) => localize_or_default(&message, bundle),
//             Self::Pattern(message) => localize_or_default(&message, bundle),
//             Self::MinItems(message) => localize_or_default(&message, bundle),
//             Self::MaxItems(message) => localize_or_default(&message, bundle),
//             Self::UniqueItems(message) => localize_or_default(&message, bundle),
//             Self::MinProperties(message) => localize_or_default(&message, bundle),
//             Self::MaxProperties(message) => localize_or_default(&message, bundle),
//             Self::Enumerate(message) => localize_or_default(&message, bundle),
//             Self::Custom(message) => message,
//             Self::Items(message) => format!("{message}"),
//             Self::Properties(message) => format!("{message}"),
//             Self::Fluent(message) => localize(&message, bundle),
//         }
//     }
// }

// fn try_localize<E: DefaultFormat>(
//     message: &crate::validation::error::Message<E>,
//     bundle: &FluentBundle<FluentResource>,
// ) -> Result<Option<String>, Vec<FluentError>> {
//     if let Some(msg) = bundle.get_message(message.id) {
//         if let Some(pattern) = msg.value() {
//             let mut errors = vec![];
//             let args = FluentArgs::from_iter(message.args.to_owned());
//             let value = bundle
//                 .format_pattern(pattern, Some(&args), &mut errors)
//                 .to_string();

//             if !errors.is_empty() {
//                 return Ok(Some(value));
//             } else {
//                 return Err(errors);
//             }
//         }
//     }

//     Ok(None)
// }

// fn localize<E: DefaultFormat>(
//     message: &crate::validation::error::Message<E>,
//     bundle: &FluentBundle<FluentResource>,
// ) -> Option<String> {
//     try_localize(message, bundle).unwrap_or_else(|_| Some(message.to_string()))
// }

// fn try_localize_or_default<E: DefaultFormat>(
//     message: &crate::validation::error::Message<E>,
//     bundle: &FluentBundle<FluentResource>,
// ) -> Result<String, Vec<FluentError>> {
//     if let Some(fluent_message) = message.fluent_message() {
//         if let Some(value) = try_localize(fluent_message, bundle)? {
//             return Ok(value);
//         }
//     }

//     Ok(format!("{message}"))
// }

// fn localize_or_default<E: DefaultFormat>(
//     message: &crate::validation::error::Message<E>,
//     bundle: &FluentBundle<FluentResource>,
// ) -> String {
//     try_localize_or_default(message, bundle).unwrap_or_else(|_| message.to_string())
// }

// #[cfg(test)]
// mod test {
//     use crate::{fluent::Message, validation::error::Format};

//     use super::*;
//     use fluent_0::{FluentResource, FluentValue};
//     use serde_valid_literal::Number;
//     use unic_langid::LanguageIdentifier;

//     #[test]
//     fn try_localize_without_args() {
//         let ftl_string = "hello-world = Hello, world!".to_string();
//         let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

//         let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
//         let mut bundle = FluentBundle::new(vec![langid_en]);
//         bundle.add_resource(res).unwrap();

//         let error = crate::validation::Error::Fluent(Message {
//             id: "hello-world",
//             args: vec![],
//         });

//         assert_eq!(error.try_localize(&bundle), Ok("Hello, world!".to_string()));
//     }

//     #[test]
//     fn try_localize_with_args() {
//         let ftl_string = "intro = Welcome, { $name }.".to_string();
//         let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

//         let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
//         let mut bundle = FluentBundle::new(vec![langid_en]);
//         bundle.add_resource(res).unwrap();

//         let error = crate::validation::Error::Fluent(Message {
//             id: "intro",
//             args: vec![("name", FluentValue::from("John"))],
//         });

//         assert_eq!(
//             error.try_localize(&bundle),
//             Ok("Welcome, \u{2068}John\u{2069}.".to_string())
//         );
//     }

//     #[test]
//     fn try_localize_from_validation_error() {
//         let ftl_string = "intro = Welcome, { $name }.".to_string();
//         let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

//         let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
//         let mut bundle = FluentBundle::new(vec![langid_en]);
//         bundle.add_resource(res).unwrap();

//         let error = crate::validation::Error::Maximum(
//             Format::Fluent(Message {
//                 id: "intro",
//                 args: vec![("name", FluentValue::from("John"))],
//             })
//             .into_message(crate::MaximumError {
//                 maximum: Number::I32(10),
//             }),
//         );

//         assert_eq!(
//             error.try_localize(&bundle),
//             Ok("Welcome, \u{2068}John\u{2069}.".to_string())
//         );
//     }
// }
