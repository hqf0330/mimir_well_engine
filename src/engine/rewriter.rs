//! SQL 重写器 - 将语义 SQL 转换为实际 SQL

/// SQL 重写器
/// 参考 wren-engine 的 Rewriter 类
pub struct Rewriter;

impl Rewriter {
    /// 创建新的重写器
    pub fn new() -> Self {
        Self
    }
}

impl Default for Rewriter {
    fn default() -> Self {
        Self::new()
    }
}
