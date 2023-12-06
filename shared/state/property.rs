use std::fmt::{Formatter, Display};
use serde::{Serialize, Deserialize};

use super::Partial;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Property {
    property: String,
    value: String,
}

impl Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "property={}, value={}", self.property, self.value)
    }
}

impl Default for Property {
    fn default() -> Self {
        Self::new("", "")
    }
}

impl Property {
    pub fn new(property: &str, value: &str) -> Self {
        Property {
            property: String::from(property),
            value: String::from(value),
        }
    }

    pub fn property(&self) -> &str {
        &self.property
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PartialProperty {
    property: Option<String>,
    value: Option<String>,
}

impl Display for PartialProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "property={}, value={}", 
            self.property.clone().unwrap_or(String::from("None")), 
            self.value.clone().unwrap_or(String::from("None"))
        )
    }
}

impl Partial<Property> for PartialProperty {
    fn merge(self, property: &Property) -> Property {
        Property {
            property: self.property.unwrap_or(property.property.clone()),
            value: self.value.unwrap_or(property.value.clone()),
        }
    }
}