use error_stack::Context;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct S3Error;

impl Display for S3Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str("S3 error")
    }
}

impl Context for S3Error {}
