//! API 层 - HTTP 路由和处理器

pub mod v3;

use axum::Router;

/// 创建主 API 路由
pub fn router() -> Router {
    Router::new().nest("/", v3::router())
}
