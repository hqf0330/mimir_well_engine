//! 响应模型

use serde::{Deserialize, Serialize};

/// 查询响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResponse {
    /// 查询结果数据
    pub data: Vec<serde_json::Value>,
    /// 列元数据
    pub columns: Vec<ColumnInfo>,
}

/// 列信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
}

/// 规划响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DryPlanResponse {
    /// 规划后的 SQL
    pub sql: String,
}
