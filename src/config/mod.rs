//! 配置模块

use serde::{Deserialize, Serialize};

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// 服务器配置
    pub server: ServerConfig,
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// 服务器端口
    pub port: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerConfig { port: 8080 },
        }
    }
}
