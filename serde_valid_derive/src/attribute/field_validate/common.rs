mod lit;
mod message_format;

pub use lit::{get_lit, get_numeric, get_str};
pub use message_format::{default_message_format, extract_custom_message_format, MessageFormat};

macro_rules! count {
    () => (0usize);
    ( $x:literal $($xs:literal)* ) => (1usize + count!($($xs)*));
}

macro_rules! enum_str {
    (pub enum $name:ident {}) => {
        pub enum $name {
        }

        impl $name {
            #[allow(dead_code)]
            pub fn name(&self) -> &'static str {
                unimplemented!()
            }

            #[allow(dead_code)]
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
            #[allow(dead_code)]
            pub fn name(&self) -> &'static str {
                match *self {
                    $($name::$variant => $val),*
                }
            }

            #[allow(dead_code)]
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
    pub enum MetaPathStructValidation {
    }
}

enum_str! {
    pub enum MetaListStructValidation {
        Custom = "custom",
    }
}

enum_str! {
    pub enum MetaNameValueStructValidation {
    }
}

enum_str! {
    pub enum MetaPathFieldValidation {
        UniqueItems = "unique_items",
    }
}

enum_str! {
    pub enum MetaListFieldValidation {
        Enumerate = "enumerate",
        Custom = "custom",
    }
}

enum_str! {
    pub enum MetaNameValueFieldValidation {
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
