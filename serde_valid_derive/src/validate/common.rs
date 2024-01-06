mod custom_message;
mod lit;

pub use custom_message::{extract_custom_message_tokens, CustomMessageToken};
pub use lit::{get_lit, get_numeric, get_str};

macro_rules! count {
    () => (0usize);
    ( $x:literal $($xs:literal)* ) => (1usize + count!($($xs)*));
}

macro_rules! enum_str {
    (pub enum $name:ident {}) => {
        pub enum $name {
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                unimplemented!()
            }

            pub fn iter() -> std::array::IntoIter<Self, 0> {
                [].into_iter()
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Err(s.to_owned())
            }
        }
    };

    (pub enum $name:ident {
        $($variant:ident = $val:literal),*,
    }) => {
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            pub fn name(&self) -> &'static str {
                match *self {
                    $($name::$variant => $val),*
                }
            }

            pub fn iter() -> std::array::IntoIter<Self, {count!($($val)*)} > {
                [
                    $($name::$variant),*
                ].into_iter()
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($val => Ok($name::$variant) ),*,
                    _ => Err(s.to_owned())
                }
            }
        }
    };
}

enum_str! {
    pub enum MetaPathValidation {
        UniqueItems = "unique_items",
    }
}

enum_str! {
    pub enum MetaListValidation {
        Enumerate = "enumerate",
        Custom = "custom",
    }
}

enum_str! {
    pub enum MetaNameValueValidation {
        Minimum = "minimum",
        Maximum = "maximum",
        ExclusiveMinimum = "exclusive_minimum",
        ExclusiveMaximum = "exclusive_maximum",
        MinLength = "min_length",
        MaxLength = "max_length",
        MinItems = "min_items",
        MaxItems = "max_items",
        MinProperties = "min_properties",
        MaxProperties = "max_properties",
        MultipleOf = "multiple_of",
        Pattern = "pattern",
    }
}

enum_str! {
    pub enum MetaPathCustomMessage {
    }
}

#[cfg(not(feature = "fluent"))]
enum_str! {
    pub enum MetaListCustomMessage {
        MessageFn = "message_fn",
    }
}

#[cfg(feature = "fluent")]
enum_str! {
    pub enum MetaListCustomMessage {
        MessageFn = "message_fn",
        I18n = "i18n",
        Fluent = "fluent",
    }
}

enum_str! {
    pub enum MetaNameValueCustomMessage {
        Message = "message",
    }
}
