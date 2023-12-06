use std::fmt::Display;

use tracing::{debug, error, info, warn};

pub trait LoggableResult<T, E> {
    fn info(self) -> Self;
    fn warn(self) -> Self;
    fn error(self) -> Self;
    fn debug(self) -> Self;
}

impl<T, E: Display> LoggableResult<T, E> for Result<T, E> {
    fn info(self) -> Self {
        match self {
            Err(e) => {
                info!("{}", e);
                Err(e)
            }
            Ok(inner) => {
                Ok(inner)
            }
        }
    }
    fn warn(self) -> Self {
        match self {
            Err(e) => {
                warn!("{}", e);
                Err(e)
            }
            ok => ok,
        }
    }
    fn error(self) -> Self {
        match self {
            Err(e) => {
                error!("{}", e);
                Err(e)
            }
            ok => ok,
        }
    }

    fn debug(self) -> Self {
        match self {
            Err(e) => {
                debug!("{}", e);
                Err(e)
            }
            Ok(inner) => {
                Ok(inner)
            }
        }
    }
}