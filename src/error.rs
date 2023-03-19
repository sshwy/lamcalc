/// Error type.
#[derive(Debug)]
pub enum Error {
    /// 化简次数超过了规定（有可能无限递归）
    SimplifyLimitExceeded,
    /// 解析 lambda 表达式时出错
    ParseError(String),
    /// 在创建 lambda 表达式时遇到无法处理的规则
    InvalidRule(String),
    /// 找不到 beta redex
    #[cfg(feature = "wasm")]
    RedexNotFound,
    /// 无效 redex
    #[cfg(feature = "wasm")]
    InvalidRedex,
    /// 无效表达式
    #[cfg(feature = "wasm")]
    InvalidDisplayExp
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SimplifyLimitExceeded => f.write_str("化简次数超过限制（可能有无限递归）"),
            Error::ParseError(msg) => write!(f, "解析错误：{}", msg),
            Error::InvalidRule(r) => write!(f, "创建 lambda 表达式时遇到无法处理的规则：{}", r),
            #[cfg(feature = "wasm")]
            Error::RedexNotFound => write!(f, "找不到 beta-redex"),
            #[cfg(feature = "wasm")]
            Error::InvalidRedex => write!(f, "无效的 redex"),
            #[cfg(feature = "wasm")]
            Error::InvalidDisplayExp => write!(f, "无效的表达式"),
        }
    }
}

impl std::error::Error for Error{}