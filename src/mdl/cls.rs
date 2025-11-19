//! Column Level Security (CLS) 相关实现
//!
//! 包含 NormalizedExpr 的实现，这些 trait 实现是 SerializeDisplay 和 DeserializeFromStr 所需要的

use crate::mdl::manifest::{NormalizedExpr, NormalizedExprType};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

impl NormalizedExpr {
    /// 创建新的 NormalizedExpr
    ///
    /// # Arguments
    /// * `expr` - 表达式字符串，如果以单引号包围则视为字符串类型，否则视为数字类型
    ///
    /// # Panics
    /// 如果表达式为空字符串会 panic
    pub fn new(expr: &str) -> Self {
        assert!(!expr.is_empty(), "expr is null or empty");

        if Self::is_string(expr) {
            NormalizedExpr {
                value: expr[1..expr.len() - 1].to_string(),
                data_type: NormalizedExprType::String,
            }
        } else {
            NormalizedExpr {
                value: expr.to_string(),
                data_type: NormalizedExprType::Numeric,
            }
        }
    }

    /// 判断表达式是否为字符串类型（以单引号开头和结尾）
    fn is_string(expr: &str) -> bool {
        expr.starts_with('\'') && expr.ends_with('\'') && expr.len() > 1
    }
}

impl Display for NormalizedExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.data_type {
            NormalizedExprType::String => write!(f, "'{}'", self.value),
            NormalizedExprType::Numeric => write!(f, "{}", self.value),
        }
    }
}

impl FromStr for NormalizedExpr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NormalizedExpr::new(s))
    }
}
