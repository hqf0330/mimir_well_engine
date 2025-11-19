//! 连接器层 - 数据库连接和执行

pub mod postgres;
pub mod trait_;

pub use postgres::PostgresConnector;
pub use trait_::Connector;
