/// Error type.
#[derive(Debug)]
pub enum Error {
    /// Too many reductions
    SimplifyLimitExceeded,
    /// Can't parse expression
    ParseError(String),
    /// Can't find beta redex. Note that this error is often recovered as we
    /// will try to find anthor beta redex in the current expression.
    #[cfg(feature = "wasm")]
    RedexNotFound,
    /// Invalid redex
    #[cfg(feature = "wasm")]
    InvalidRedex(u32, String),
    /// Invalid display expression
    #[cfg(feature = "wasm")]
    InvalidDisplayExp,
    /// Invalid inner type of display expression
    #[cfg(feature = "wasm")]
    InvalidInnerType,
    /// Can't find variable when doing replacement
    #[cfg(feature = "wasm")]
    VarNotFound(String, u32),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SimplifyLimitExceeded => f.write_str("too many reductions"),
            Error::ParseError(msg) => write!(f, "error parsing: {}", msg),
            #[cfg(feature = "wasm")]
            Error::RedexNotFound => write!(f, "redex not found"),
            #[cfg(feature = "wasm")]
            Error::InvalidRedex(id, s) => write!(f, "invalid redex(id = {}): {}", id, s),
            #[cfg(feature = "wasm")]
            Error::InvalidDisplayExp => write!(f, "invalid display expression"),
            #[cfg(feature = "wasm")]
            Error::InvalidInnerType => write!(f, "invalid inner type of display expression"),
            #[cfg(feature = "wasm")]
            Error::VarNotFound(name, id) => {
                write!(f, "free variable \"{}\" not found, alpha_id = {}", name, id)
            }
        }
    }
}

impl std::error::Error for Error {}
