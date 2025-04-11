use std::fmt;
use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug)]
pub struct CacheError;

impl Display for CacheError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str("Cache error")
    }
}

impl Context for CacheError {}