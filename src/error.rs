use std::fmt::{Display, Formatter};
use failure::{Backtrace, Context, Fail};

#[derive(Debug)]
pub struct KvsError {
    inner: Context<KvsErrorKind>
}

#[derive(Clone, Debug, Eq, PartialEq, Copy, Fail)]
pub enum KvsErrorKind {
    #[fail(display = "Just a general error")]
    GeneralError
}

impl Display for KvsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Fail for KvsError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}


// Conversions for the error types
impl KvsError {
    pub fn kind(&self) -> KvsErrorKind {
        *self.inner.get_context()
    }
}

impl From<KvsErrorKind> for KvsError {
    fn from(value: KvsErrorKind) -> Self {
        KvsError {inner: Context::new(value)}
    }
}

impl From<Context<KvsErrorKind>> for KvsError {
    fn from(value: Context<KvsErrorKind>) -> Self {
        KvsError {inner: value}
    }
}

/// The general error definition for this crate
pub type Result<T> = std::result::Result<T, KvsError>;

