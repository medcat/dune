use failure::{Context, Fail};

#[derive(Debug)]
pub struct DocumentError(Context<DocumentErrorKind>);

impl DocumentError {
    pub fn kind(&self) -> &DocumentErrorKind {
        self.0.get_context()
    }
}

impl Fail for DocumentError {
    fn name(&self) -> Option<&str> {
        self.0.name()
    }
    fn cause(&self) -> Option<&dyn Fail> {
        self.0.cause()
    }
    fn backtrace(&self) -> Option<&failure::Backtrace> {
        self.0.backtrace()
    }
}

impl std::fmt::Display for DocumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Fail)]
pub enum DocumentErrorKind {
    #[fail(display = "could not serialize document")]
    SerializeError,
}

impl From<DocumentErrorKind> for DocumentError {
    fn from(kind: DocumentErrorKind) -> Self {
        DocumentError(Context::new(kind))
    }
}

impl From<Context<DocumentErrorKind>> for DocumentError {
    fn from(context: Context<DocumentErrorKind>) -> Self {
        DocumentError(context)
    }
}
