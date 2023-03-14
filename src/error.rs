#[derive(Debug)]

/// 处理 lambda 表达式过程中产生的错误
pub enum Error {
    /// 化简次数超过了规定（有可能无限递归）
    SimplifyLimitExceeded,
    /// 解析 lambda 表达式时出错
    ParseError(String),
    /// 在创建 lambda 表达式时遇到无法处理的规则
    InvalidRule(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SimplifyLimitExceeded => f.write_str("化简次数超过限制（可能有无限递归）"),
            Error::ParseError(msg) => write!(f, "解析错误：{}", msg),
            Error::InvalidRule(r) => write!(f, "创建 lambda 表达式时遇到无法处理的规则：{}", r),
        }
    }
}

impl std::error::Error for Error{}