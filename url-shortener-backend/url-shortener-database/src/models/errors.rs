use error_stack::Context;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct DatabaseError;

impl Display for DatabaseError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str("Database error")
    }
}

impl Context for DatabaseError {}
