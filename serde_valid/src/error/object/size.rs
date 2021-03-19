use crate::traits::Size;

#[derive(Debug)]
pub struct PropertiesErrorInfo {
    properties: String,
    properties_size: usize,
    min_properties: Option<usize>,
    max_properties: Option<usize>,
}

impl PropertiesErrorInfo {
    pub fn new<T>(
        properties: &T,
        min_properties: Option<usize>,
        max_properties: Option<usize>,
    ) -> Self
    where
        T: std::fmt::Debug + Size,
    {
        Self {
            properties: format!("{:?}", properties),
            properties_size: properties.size(),
            min_properties,
            max_properties,
        }
    }
}

impl std::fmt::Display for PropertiesErrorInfo {
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
