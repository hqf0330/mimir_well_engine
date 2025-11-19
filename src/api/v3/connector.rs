//! v3 Connector API - 数据源连接器接口

use crate::error::Error;
use crate::model::{DryPlanRequest, QueryRequest};
use axum::{
    extract::Path,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};

/// 创建 v3 connector 路由
pub fn router() -> Router {
    Router::new()
        .route("/v3/connector/:data_source/query", post(query))
        .route("/v3/connector/:data_source/dry-plan", post(dry_plan))
        .route("/health", get(health))
}

/// 查询接口 - 执行 SQL 查询
/// POST /v3/connector/{data_source}/query
async fn query(
    Path(_data_source): Path<String>,
    Json(_request): Json<QueryRequest>,
) -> impl IntoResponse {
    // TODO: 实现查询逻辑
    let err = Error::Planning("Not implemented yet".to_string());
    (
        axum::http::StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "error": "Not implemented",
            "message": err.to_string()
        })),
    )
}

/// 规划接口 - SQL 规划（不执行）
/// POST /v3/connector/{data_source}/dry-plan
async fn dry_plan(
    Path(_data_source): Path<String>,
    Json(_request): Json<DryPlanRequest>,
) -> impl IntoResponse {
    // TODO: 实现 SQL 规划逻辑
    let err = Error::Planning("Not implemented yet".to_string());
    (
        axum::http::StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "error": "Not implemented",
            "message": err.to_string()
        })),
    )
}

/// 健康检查
/// GET /health
async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}
