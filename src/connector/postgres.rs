//! PostgreSQL 连接器

use crate::connector::trait_::Connector;
use crate::error::Result;
use crate::model::{ConnectionInfo, QueryResponse};

/// PostgreSQL 连接器
pub struct PostgresConnector {
    _connection_info: ConnectionInfo,
}

impl PostgresConnector {
    /// 创建新的 PostgreSQL 连接器
    pub fn new(connection_info: ConnectionInfo) -> Self {
        Self {
            _connection_info: connection_info,
        }
    }
}

#[async_trait::async_trait]
impl Connector for PostgresConnector {
    async fn query(&self, _sql: &str) -> Result<QueryResponse> {
        // TODO: 实现 PostgreSQL 查询执行
        Err(crate::error::Error::Connector(
            "Not implemented yet".to_string(),
        ))
    }

    fn name(&self) -> &str {
        "postgres"
    }
}
