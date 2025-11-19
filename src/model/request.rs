//! 请求模型 (DTO)

use serde::{Deserialize, Serialize};

/// 查询请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    /// SQL 查询语句
    pub sql: String,
    /// MDL manifest (base64 编码的 JSON)
    pub manifest_str: String,
    /// 连接信息
    pub connection_info: ConnectionInfo,
}

/// 规划请求（不执行查询）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryPlanRequest {
    /// SQL 查询语句
    pub sql: String,
    /// MDL manifest (base64 编码的 JSON)
    pub manifest_str: String,
    /// 连接信息（可选）
    pub connection_info: Option<ConnectionInfo>,
}

/// 数据库连接信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
    pub schema: Option<String>,
}
