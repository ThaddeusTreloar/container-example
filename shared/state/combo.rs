use std::fmt::{Display, Formatter};

use super::{entity::Entity, property::Property, Partial};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Combo {
    origin: String,
    colour: String,
    property: String,
    value: String,
}

impl Display for Combo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "origin={}, colour={}, property={}, value={}",
            self.origin, self.colour, self.property, self.value
        )
    }
}

impl Combo {
    pub fn new(origin: &str, colour: &str, property: &str, value: &str) -> Self {
        Combo {
            origin: String::from(origin),
            colour: String::from(colour),
            property: String::from(property),
            value: String::from(value),
        }
    }

    pub fn origin(&self) -> &str {
        &self.origin
    }

    pub fn colour(&self) -> &str {
        &self.colour
    }

    pub fn property(&self) -> &str {
        &self.property
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Into<Entity> for Combo {
    fn into(self) -> Entity {
        Entity::new(self.origin(), self.colour())
    }
}

impl Into<Property> for Combo {
    fn into(self) -> Property {
        Property::new(self.property(), self.value())
    }
}

impl From<(Entity, Property)> for Combo {
    fn from((entity, property): (Entity, Property)) -> Self {
        Combo::new(
            entity.origin(),
            entity.colour(),
            property.property(),
            property.value(),
        )
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MaybeCombo {
    origin: String,
    colour: String,
    property: Option<String>,
    value: Option<String>,
}

impl From<Entity> for MaybeCombo {
    fn from(entity: Entity) -> Self {
        MaybeCombo {
            origin: entity.origin().to_string(),
            colour: entity.colour().to_string(),
            property: None,
            value: None,
        }
    }
}

impl TryInto<Property> for MaybeCombo {
    type Error = String;

    fn try_into(self) -> Result<Property, Self::Error> {
        if self.property.is_none() {
            return Err(String::from("property is None"));
        }

        if self.value.is_none() {
            return Err(String::from("value is None"));
        }

        Ok(Property::new(
            self.property.unwrap().as_str(),
            self.value.unwrap().as_str(),
        ))
    }
}

impl Display for MaybeCombo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "origin={}, colour={}, property={}, value={}",
            self.origin.clone(),
            self.colour.clone(),
            self.property.clone().unwrap_or(String::from("None")),
            self.value.clone().unwrap_or(String::from("None"))
        )
    }
}

impl Partial<Combo> for MaybeCombo {
    fn merge(self, entity: &Combo) -> Combo {
        Combo {
            origin: self.origin,
            colour: self.colour,
            property: self.property.unwrap_or(entity.property.clone()),
            value: self.value.unwrap_or(entity.value.clone()),
        }
    }
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PartialCombo {
    origin: Option<String>,
    colour: Option<String>,
    property: Option<String>,
    value: Option<String>,
}

impl Display for PartialCombo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "origin={}, colour={}, property={}, value={}",
            self.origin.clone().unwrap_or(String::from("None")),
            self.colour.clone().unwrap_or(String::from("None")),
            self.property.clone().unwrap_or(String::from("None")),
            self.value.clone().unwrap_or(String::from("None"))
        )
    }
}

impl Partial<Combo> for PartialCombo {
    fn merge(self, entity: &Combo) -> Combo {
        Combo {
            origin: self.origin.unwrap_or(entity.origin.clone()),
            colour: self.colour.unwrap_or(entity.colour.clone()),
            property: self.property.unwrap_or(entity.property.clone()),
            value: self.value.unwrap_or(entity.value.clone()),
        }
    }
}
