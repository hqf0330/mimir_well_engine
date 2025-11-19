//! MDL Manifest 模型定义

// 使用宏生成 SessionProperty 和 RowLevelAccessControl
mod manifest_impl {
    use mdl_macro::{row_level_access_control, session_property};
    use serde::{Deserialize, Serialize};

    session_property!();
    row_level_access_control!();
}

// 导出宏生成的结构体
pub use manifest_impl::{RowLevelAccessControl, SessionProperty};

#[cfg(test)]
mod tests {
    use super::SessionProperty;
    use serde_json;

    #[test]
    fn test_session_property_new() {
        let prop = SessionProperty::new("UserId".to_string(), true, Some("123".to_string()));

        assert_eq!(prop.name, "UserId");
        assert_eq!(prop.required, true);
        assert_eq!(prop.default_expr, Some("123".to_string()));
        assert_eq!(prop.normalized_name, "userid");
    }

    #[test]
    fn test_session_property_normalized_name() {
        let prop = SessionProperty::new("MyProperty".to_string(), false, None);
        assert_eq!(prop.normalized_name(), "myproperty");
    }

    #[test]
    fn test_session_property_deserialize() {
        let json = r#"{
            "name": "UserId",
            "required": true,
            "defaultExpr": "123"
        }"#;

        let prop: SessionProperty = serde_json::from_str(json).unwrap();
        assert_eq!(prop.name, "UserId");
        assert_eq!(prop.required, true);
        assert_eq!(prop.default_expr, Some("123".to_string()));
        assert_eq!(prop.normalized_name, "userid");
    }

    #[test]
    fn test_session_property_deserialize_without_default_expr() {
        let json = r#"{
            "name": "UserId",
            "required": false
        }"#;

        let prop: SessionProperty = serde_json::from_str(json).unwrap();
        assert_eq!(prop.name, "UserId");
        assert_eq!(prop.required, false);
        assert_eq!(prop.default_expr, None);
        assert_eq!(prop.normalized_name, "userid");
    }

    #[test]
    fn test_session_property_serialize() {
        let prop = SessionProperty::new("UserId".to_string(), true, Some("123".to_string()));

        let json = serde_json::to_string(&prop).unwrap();
        assert!(json.contains("\"name\":\"UserId\""));
        assert!(json.contains("\"required\":true"));
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
}
