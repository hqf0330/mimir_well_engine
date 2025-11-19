//! MDL Manifest 模型定义

use serde::{Deserialize, Serialize};

/// MDL Manifest 结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub catalog: String,
    pub schema: String,
    pub models: Vec<Model>,
    pub relationships: Vec<Relationship>,
}

/// 模型定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub table_reference: TableReference,
    pub columns: Vec<Column>,
}

/// 表引用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableReference {
    pub catalog: String,
    pub schema: String,
    pub table: String,
}

/// 列定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub r#type: String,
}

/// 关系定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub name: String,
    pub models: Vec<String>,
    pub join_type: String,
    pub condition: String,
}
