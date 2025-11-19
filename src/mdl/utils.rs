use sqlparser::{ast::Ident, dialect::GenericDialect, parser::Parser, parser::ParserError};

/// 解析 SQL 多部分标识符
pub(crate) fn parse_identifiers(s: &str) -> Result<Vec<Ident>, ParserError> {
    let dialect = GenericDialect;
    let mut parser = Parser::new(&dialect).try_with_sql(s)?;
    let idents = parser.parse_multipart_identifier()?;
    Ok(idents)
}

pub(crate) fn parse_identifiers_normalized(
    s: &str,
    ignore_case: bool,
) -> Result<Vec<String>, ParserError> {
    parse_identifiers(s).map(|v| {
        v.into_iter()
            .map(|id| match id.quote_style {
                Some(_) => id.value,
                None if ignore_case => id.value,
                _ => id.value.to_ascii_lowercase(),
            })
            .collect::<Vec<_>>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_identifiers_quoted() {
        let input = r#""Catalog"."Schema"."Table""#;
        let result = parse_identifiers(input).unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].value, "Catalog");
        assert_eq!(result[0].quote_style, Some('"'));
        assert_eq!(result[1].value, "Schema");
        assert_eq!(result[1].quote_style, Some('"'));
        assert_eq!(result[2].value, "Table");
        assert_eq!(result[2].quote_style, Some('"'));
    }

    #[test]
    fn test_parse_identifiers_unquoted() {
        let input = "catalog.schema.table";
        let result = parse_identifiers(input).unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].value, "catalog");
        assert_eq!(result[0].quote_style, None);
        assert_eq!(result[1].value, "schema");
        assert_eq!(result[1].quote_style, None);
        assert_eq!(result[2].value, "table");
        assert_eq!(result[2].quote_style, None);
    }

    #[test]
    fn test_parse_identifiers_normalized_quoted() {
        let input = r#""Catalog"."Schema"."Table""#;
        let result = parse_identifiers_normalized(input, false).unwrap();

        // 带引号的标识符应该保留大小写
        assert_eq!(result, vec!["Catalog", "Schema", "Table"]);
    }

    #[test]
    fn test_parse_identifiers_normalized_unquoted() {
        let input = "Catalog.Schema.Table";
        let result = parse_identifiers_normalized(input, false).unwrap();

        // 不带引号的标识符应该转为小写
        assert_eq!(result, vec!["catalog", "schema", "table"]);
    }

    #[test]
    fn test_parse_identifiers_single_part() {
        let input = r#""MyTable""#;
        let result = parse_identifiers(input).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].value, "MyTable");
        assert_eq!(result[0].quote_style, Some('"'));
    }
}
