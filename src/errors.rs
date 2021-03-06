use resources::Status;
use std::fmt::{self, Display};
use failure::{Backtrace, Context, Fail};

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        self.inner.get_context().clone()
    }

    pub fn is_kubernetes_status(&self) -> bool {
        if let &ErrorKind::Status(_, _) = self.inner.get_context() {
            true
        } else {
            false
        }
    }

    pub fn kubernetes_status(&self) -> Option<&Status> {
        if let &ErrorKind::Status(_, ref s) = self.inner.get_context() {
            Some(s)
        } else {
            None
        }
    }

    pub fn http_status(&self) -> Option<u16> {
        if let &ErrorKind::Status(s, _) = self.inner.get_context() {
            Some(s)
        } else {
            None
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { inner: Context::new(kind) }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner: inner }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Failed to build Pkcs12")]
    Pkcs,
    #[fail(display = "Failed to build reqwest client")]
    ReqwestInit,
    #[fail(display = "Failed to build URL")]
    Url,
    #[fail(display = "HTTP request failed")]
    Request,
    #[fail(display = "Failed to parse JSON response")]
    Json,
    #[fail(display = "Failed to read resource file")]
    ResourceFileIo,
    #[fail(display = "Failed to parse resource file")]
    ResourceFileParsing,
    #[fail(display = "Kubernetes status: {}", _0)]
    Status(u16, Status),
    #[fail(display = "Failed to load Kubernetes config file")]
    Config,
    #[fail(display = "Failed to find the chosen context")]
    ConfigContext, // TODO more detailed errors here
}

pub type Result<T> = ::std::result::Result<T, Error>;
