//! Mimir Well Engine
//!
//! 语义层引擎服务 - 基于 Rust 和 Axum 构建
//! 参考 wren-engine 架构设计

pub mod api;
pub mod config;
pub mod connector;
pub mod engine;
pub mod error;
pub mod mdl;
pub mod model;

// 重新导出常用类型
pub use error::{Error, Result};
