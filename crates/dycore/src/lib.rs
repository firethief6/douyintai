#![recursion_limit = "256"]
//! 端内数据层：身份池、请求装配、信封签名、抖音接口。
pub mod api;
pub mod diag;
pub mod http;
pub mod params;
pub mod pb;
pub mod pool;
pub mod proxy;
pub mod register;
pub mod signer;
pub mod spade;

use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub status: u16,
    pub message: String,
    /// 前端据此给可操作的提示；只用于需要用户决策的错误。
    pub kind: Option<&'static str>,
}

impl Error {
    pub fn new(status: u16, message: impl Into<String>) -> Self {
        Self {
            status,
            kind: None,
            message: message.into(),
        }
    }
    pub fn msg(message: impl Into<String>) -> Self {
        Self::new(0, message)
    }
    /// 带机器可读类别的错误，前端按类别给操作入口。
    pub fn kinded(kind: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: 0,
            kind: Some(kind),
            message: message.into(),
        }
    }

    /// 换一个身份值得重试的错误：网络、签名拒绝、限流、上游契约抖动、签名服务不可用。
    /// 参数错误、需要登录、内容不存在重试也不会变好。
    pub fn retryable(&self) -> bool {
        matches!(self.status, 0 | 403 | 429 | 502 | 503)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}
impl std::error::Error for Error {}
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::new(e.status().map(|s| s.as_u16()).unwrap_or(0), e.to_string())
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::new(0, e.to_string())
    }
}
impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Self::new(400, e.to_string())
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::new(0, e.to_string())
    }
}
impl From<Box<dyn std::error::Error>> for Error {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        Self::new(0, e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
