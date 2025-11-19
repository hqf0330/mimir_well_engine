//! Manifest 宏定义
//!
//! 用于生成 MDL 相关的结构体定义

use quote::quote;

#[proc_macro]
pub fn manifest(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
   let expanded = quote! {

        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
        #[serde(rename_all = "camelCase")]
        pub struct Manifest {
            pub catalog: String,
            pub schema: String,
            #[serde(default)]
            pub models: Vec<Arc<Model>>,
            #[serde(default)]
            pub relationships: Vec<Arc<Relationship>>,
            #[serde(default)]
            pub metrics: Vec<Arc<Metric>>,
            #[serde(default)]
            pub views: Vec<Arc<View>>,
            #[serde(default)]
            pub data_source: Option<DataSource>,
        }
   };
   proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn relationship(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expanded = quote! {
        #[serde_as]
        #[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        pub struct Relationship {
            pub name: String,
            pub models: Vec<String>,
            pub join_type: JoinType,
            pub condition: String,
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn join_type(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum JoinType {
            #[serde(alias = "one_to_one")]
            OneToOne,
            #[serde(alias = "one_to_many")]
            OneToMany,
            #[serde(alias = "many_to_one")]
            ManyToOne,
            #[serde(alias = "many_to_many")]
            ManyToMany,
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn metric(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let expanded = quote! {
        #[serde_as]
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
        #[serde(rename_all = "camelCase")]
        pub struct Metric {
            pub name: String,
            pub base_object: String,
            pub dimension: Vec<Arc<Column>>,
            pub measure: Vec<Arc<Column>>,
            pub time_grain: Vec<TimeGrain>,
            #[serde(default, with = "bool_from_int")]
            pub cached: bool,
            pub refresh_time: Option<String>,
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn time_grain(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
        #[serde(rename_all = "camelCase")]
        pub struct TimeGrain {
            pub name: String,
            pub ref_column: String,
            pub date_parts: Vec<TimeUnit>,
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn time_unit(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
        pub enum TimeUnit {
            Year,
            Month,
            Day,
            Hour,
            Minute,
            Second,
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn view(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
        pub struct View {
            pub name: String,
            pub statement: String,
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn data_source(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
        #[serde(rename_all = "UPPERCASE")]
        pub enum DataSource {
            #[serde(alias = "mysql")]
            MySQL,
            #[default]
            #[serde(alias = "datafusion")]
            Datafusion,
            #[serde(alias = "postgres")]
            Postgres,
            #[serde(alias = "duckdb")]
            DuckDB,
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn model(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
        #[serde(rename_all = "camelCase")]
        pub struct Model {
            pub name: String,

            #[serde(default)]
            pub ref_sql: Option<String>,

            #[serde(default)]
            pub base_object: Option<String>,

            #[serde(default, with = "table_reference")]
            pub table_reference: Option<String>,
            pub columns: Vec<Arc<Column>>,

            #[serde(default)]
            pub primary_key: Option<String>,

            #[serde(default, with = "bool_from_int")]
            pub cached: bool,

            #[serde(default)]
            pub refresh_time: Option<String>,

            #[serde(default)]
            pub row_level_access_controls: Vec<Arc<RowLevelAccessControl>>,
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn column(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expanded = quote! {

        #[serde_as]
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
        #[serde(rename_all = "camelCase")]
        #[allow(deprecated)]
        pub struct Column {
            pub name: String,
            pub r#type: String,
            #[serde(default)]
            pub relationship: Option<String>,
            #[serde(default, with = "bool_from_int")]
            pub is_calculated: bool,
            #[serde(default, with = "bool_from_int")]
            pub not_null: bool,
            #[serde_as(as = "NoneAsEmptyString")]
            #[serde(default)]
            pub expression: Option<String>,
            #[serde(default, with = "bool_from_int")]
            pub is_hidden: bool,
            pub column_level_access_control: Option<Arc<ColumnLevelAccessControl>>
        }

    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn column_level_access_control(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
        #[serde(rename_all = "camelCase")]
        pub struct ColumnLevelAccessControl {
            pub name: String,
            pub required_properties: Vec<SessionProperty>,
            pub operator: ColumnLevelOperator,
            pub threshold: NormalizedExpr,
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn column_level_operator(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum ColumnLevelOperator {
            Equals,
            NotEquals,
            GreaterThan,
            LessThan,
            GreaterThanOrEquals,
            LessThanOrEquals,
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn normalized_expr(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expanded = quote! {
        #[derive(SerializeDisplay, DeserializeFromStr, Debug, PartialEq, Eq, Hash)]
        pub struct NormalizedExpr {
            pub value: String,
            #[serde_with(alias = "type")]
            pub data_type: NormalizedExprType,
        }
    };
    proc_macro::TokenStream::from(expanded)
}


#[proc_macro]
pub fn normalized_expr_type(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum NormalizedExprType {
            Numeric,
            String,
        }
    };
    proc_macro::TokenStream::from(expanded)
}


#[proc_macro]
pub fn session_property(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expanded = quote! {
        #[derive(Serialize, Debug, PartialEq, Eq, Hash, Clone)]
        #[serde(rename_all = "camelCase")]
        pub struct SessionProperty {
            pub name: String,
            pub required: bool,
            pub default_expr: Option<String>,
            // 避免重复克隆，存储规范化名称（小写）
            #[serde(skip_serializing, default = "String::new")]
            pub normalized_name: String,
        }

        impl SessionProperty {
            /// 创建新的 SessionProperty
            pub fn new(name: String, required: bool, default_expr: Option<String>) -> Self {
                let normalized_name = name.to_lowercase();
                Self {
                    name,
                    required,
                    default_expr,
                    normalized_name,
                }
            }

            /// 获取规范化名称
            pub fn normalized_name(&self) -> &str {
                &self.normalized_name
            }
        }

        impl<'de> serde::Deserialize<'de> for SessionProperty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(rename_all = "camelCase")]
                struct SessionPropertyHelper {
                    name: String,
                    required: bool,
                    default_expr: Option<String>,
                }

                let helper = SessionPropertyHelper::deserialize(deserializer)?;
                Ok(SessionProperty {
                    normalized_name: helper.name.to_lowercase(),
                    name: helper.name,
                    required: helper.required,
                    default_expr: helper.default_expr,
                })
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn row_level_access_control(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expanded = quote! {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
        #[serde(rename_all = "camelCase")]
        pub struct RowLevelAccessControl {
            pub name: String,
            #[serde(default)]
            pub required_properties: Vec<SessionProperty>,
            /// 可评估为布尔值的字符串表达式
            pub condition: String,
        }
    };
    proc_macro::TokenStream::from(expanded)
}
