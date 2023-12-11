use std::{collections::HashMap, sync::Arc, fmt::Display};

use tokio::sync::Mutex;
use tracing::{info, warn};

pub mod property;
pub mod entity;
pub mod combo;

#[derive(Debug, Clone)]
pub struct AppState<T> {
    state: Arc::<Mutex<HashMap<String, T>>>,
}

impl <T> Default for AppState<T> 
where T: Clone + Display
{
    fn default() -> Self {
        Self::new()
    }
}

impl <T> AppState<T> 
where T: Clone + Display
{
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get(&self, key: &str) -> Option<T> {
        let state = self.state.lock().await;
        state.get(key).cloned()
    }

    pub async fn set(&self, key: &str, value: &T) -> Option<T> {
        let mut state = self.state.lock().await;
        state.insert(key.to_string(), value.clone())
    }

    pub async fn update<U: Partial<T> + Clone>(&self, key: &str, partial_value: &U) -> Option<T> {
        let mut state = self.state.lock().await;

        match self.get(key).await {
            Some(value) => {
                info!("Patch item: {key}:{value}");
                state.insert(key.to_string(), partial_value.clone().merge(&value))
            },
            None => {
                warn!("Attempted to patch item: {key} but didn't exist");
                None
            }
        }
    }

    pub async fn rm(&self, key: &str) -> Option<T> {
        let mut state = self.state.lock().await;
        
        match state.remove(key) {
            Some(value) => {
                info!("Removed item: {key}:{value}");
                Some(value)
            },
            None => {
                warn!("Attempted to remove item: {key} but didn't exist");
                None
            }
        }
    }
}

pub trait Partial<T> {
    fn merge(self, property: &T) -> T;
}
