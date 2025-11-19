//! 连接器 Trait 定义

use crate::error::Result;
use crate::model::QueryResponse;

/// 连接器 Trait
/// 定义数据库连接器的统一接口
#[async_trait::async_trait]
pub trait Connector: Send + Sync {
    /// 执行 SQL 查询
    async fn query(&self, sql: &str) -> Result<QueryResponse>;

    /// 获取连接器名称
    fn name(&self) -> &str;
}
