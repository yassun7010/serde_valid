use crate::traits::{Properties, Size};

#[derive(Debug, serde::Serialize)]
pub struct PropertiesErrorMessage {
    properties: String,
    properties_size: usize,
    min_properties: Option<usize>,
    max_properties: Option<usize>,
}

impl PropertiesErrorMessage {
    pub fn new<T>(
        properties: &T,
        min_properties: Option<usize>,
        max_properties: Option<usize>,
    ) -> Self
    where
        T: Size + Properties,
    {
        Self {
            properties: properties.to_string(),
            properties_size: properties.size(),
            min_properties,
            max_properties,
        }
    }

    #[allow(dead_code)]
    pub fn properties(&self) -> &String {
        &self.properties
    }

    #[allow(dead_code)]
    pub fn properties_size(&self) -> usize {
        self.properties_size
    }

    #[allow(dead_code)]
    pub fn min_properties(&self) -> Option<usize> {
        self.min_properties
    }

    #[allow(dead_code)]
    pub fn max_properties(&self) -> Option<usize> {
        self.max_properties
    }
}

impl std::fmt::Display for PropertiesErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_properties = match &self.min_properties {
            Some(properties) => format!("{} <= ", properties),
            None => String::new(),
        };
        let max_properties = match &self.max_properties {
            Some(properties) => format!(" <= {}", properties),
            None => String::new(),
        };
        write!(
            f,
            "properties size of {} must be in `{}size{}`, but `{}`.",
            self.properties, min_properties, max_properties, self.properties_size
        )
    }
}
