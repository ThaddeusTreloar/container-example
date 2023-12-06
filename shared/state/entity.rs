use std::fmt::{Formatter, Display};

use super::Partial;


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Entity {
    origin: String,
    colour: String,
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "origin={}, colour={}", self.origin, self.colour)
    }
}

impl Entity {
    pub fn new(origin: &str, colour: &str) -> Self {
        Entity {
            origin: String::from(origin),
            colour: String::from(colour),
        }
    }

    pub fn origin(&self) -> &str {
        &self.origin
    }

    pub fn colour(&self) -> &str {
        &self.colour
    }
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PartialEntity {
    origin: Option<String>,
    colour: Option<String>,
}

impl Display for PartialEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "origin={}, colour={}", 
            self.origin.clone().unwrap_or(String::from("None")), 
            self.colour.clone().unwrap_or(String::from("None"))
        )
    }
}

impl Partial<Entity> for PartialEntity {
    fn merge(self, entity: &Entity) -> Entity {
        Entity {
            origin: self.origin.unwrap_or(entity.origin.clone()),
            colour: self.colour.unwrap_or(entity.colour.clone()),
        }
    }
}