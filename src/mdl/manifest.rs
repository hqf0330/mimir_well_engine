//! MDL Manifest 模型定义

// bool_from_int 模块：用于将整数 (0/1) 转换为布尔值
// 这是为了向后兼容旧格式的 MDL，其中布尔值用整数表示
mod bool_from_int {
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        match value {
            serde_json::Value::Bool(b) => Ok(b),
            // 向后兼容：旧格式用整数 (0 或 1) 表示布尔值
            serde_json::Value::Number(n) if n.is_u64() => Ok(n.as_u64().unwrap() != 0),
            _ => Err(serde::de::Error::custom("invalid type for boolean")),
        }
    }

    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Serialize::serialize(value, serializer)
    }
}

// table_reference 模块：用于序列化/反序列化表引用
// 支持格式: "catalog.schema.table" 或 "schema.table" 或 "table"
mod table_reference {
    use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

    use crate::mdl::utils::parse_identifiers_normalized;

    #[derive(Deserialize, Serialize, Default)]
    struct TableReference {
        catalog: Option<String>,
        schema: Option<String>,
        table: Option<String>,
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Option::deserialize(deserializer)?
            .map(
                |TableReference {
                     catalog,
                     schema,
                     table,
                 }| {
                    [catalog, schema, table]
                        .into_iter()
                        .flatten()
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                        .join(".")
                },
            )
            .filter(|s| !s.is_empty()))
    }

    pub fn serialize<S>(table_ref: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(table_ref) = table_ref {
            let parts = parse_identifiers_normalized(table_ref, false).map_err(|e| {
                serde::ser::Error::custom(format!(
                    "Failed to parse table reference: {table_ref}, error: {e}"
                ))
            })?;

            if parts.len() > 3 {
                return Err(serde::ser::Error::custom(format!(
                    "Invalid table reference: {table_ref}"
                )));
            }

            let table_ref = if parts.len() == 3 {
                TableReference {
                    catalog: Some(parts[0].to_string()),
                    schema: Some(parts[1].to_string()),
                    table: Some(parts[2].to_string()),
                }
            } else if parts.len() == 2 {
                TableReference {
                    catalog: None,
                    schema: Some(parts[0].to_string()),
                    table: Some(parts[1].to_string()),
                }
            } else if parts.len() == 1 {
                TableReference {
                    catalog: None,
                    schema: None,
                    table: Some(parts[0].to_string()),
                }
            } else {
                TableReference::default()
            };

            table_ref.serialize(serializer)
        } else {
            serializer.serialize_none()
        }
    }
}

// 使用宏生成 SessionProperty 和 RowLevelAccessControl
mod manifest_impl {
    use crate::mdl::manifest::bool_from_int;
    use crate::mdl::manifest::table_reference;
    use mdl_macro::{
        column, column_level_access_control, column_level_operator, data_source, join_type,
        manifest, metric, model, normalized_expr, normalized_expr_type, relationship,
        row_level_access_control, session_property, time_grain, time_unit, view,
    };
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;
    use serde_with::DeserializeFromStr;
    use serde_with::NoneAsEmptyString;
    use serde_with::SerializeDisplay;
    use std::sync::Arc;

    data_source!();
    model!();
    column!();
    relationship!();
    metric!();
    view!();
    join_type!();
    time_grain!();
    time_unit!();
    manifest!();
    row_level_access_control!();
    column_level_access_control!();
    session_property!();
    normalized_expr!();
    normalized_expr_type!();
    column_level_operator!();
}

// 导出宏生成的结构体
pub use manifest_impl::{
    Column, ColumnLevelAccessControl, ColumnLevelOperator, DataSource, JoinType, Manifest, Metric,
    Model, NormalizedExpr, NormalizedExprType, Relationship, RowLevelAccessControl,
    SessionProperty, TimeGrain, TimeUnit, View,
};

#[cfg(test)]
mod tests {
    use super::SessionProperty;
    use crate::mdl::manifest::table_reference;
    use serde_json::Serializer;

    #[test]
    fn test_session_property_normalized_name() {
        let prop = SessionProperty::new("MyProperty".to_string(), false, None);
        assert_eq!(prop.normalized_name(), "myproperty");
    }

    #[test]
    fn test_session_property_serialize() {
        let prop = SessionProperty::new("UserId".to_string(), true, Some("123".to_string()));

        let json = serde_json::to_string(&prop).unwrap();
        assert!(json.contains("\"name\":\"UserId\""));
        assert!(json.contains("\"required\":"));
        assert!(json.contains("\"defaultExpr\":\"123\""));
        // normalized_name 应该被 skip_serializing，所以不应该出现在 JSON 中
        assert!(!json.contains("normalizedName"));
    }

    #[test]
    fn test_session_property_case_insensitive_normalization() {
        let test_cases = vec![
            ("UserId", "userid"),
            ("MY_PROPERTY", "my_property"),
            ("camelCase", "camelcase"),
            ("PascalCase", "pascalcase"),
        ];

        for (input, expected) in test_cases {
            let prop = SessionProperty::new(input.to_string(), false, None);
            assert_eq!(prop.normalized_name(), expected);
        }
    }

    #[test]
    fn test_table_reference_serialize() {
        [
            (
                Some("catalog.schema.table".to_string()),
                r#"{"catalog":"catalog","schema":"schema","table":"table"}"#,
            ),
            (
                Some("schema.table".to_string()),
                r#"{"catalog":null,"schema":"schema","table":"table"}"#,
            ),
            (
                Some("table".to_string()),
                r#"{"catalog":null,"schema":null,"table":"table"}"#,
            ),
            (None, "null"),
        ]
        .iter()
        .for_each(|(table_ref, expected)| {
            let mut buf = Vec::new();
            table_reference::serialize(table_ref, &mut Serializer::new(&mut buf)).unwrap();
            assert_eq!(String::from_utf8(buf).unwrap(), *expected);
        });
    }

    #[test]
    fn test_case_sensitive() {
        let table_ref = Some(r#""Catalog"."Schema"."Table""#.to_string());
        let mut buf = Vec::new();
        table_reference::serialize(&table_ref, &mut Serializer::new(&mut buf)).unwrap();
        let serialized = String::from_utf8(buf).unwrap();
        assert_eq!(
            serialized,
            r#"{"catalog":"Catalog","schema":"Schema","table":"Table"}"#
        );
    }
}
