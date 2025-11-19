# Wren Engine 学习规划文档

> 本文档旨在帮助开发者系统性地学习 Wren Engine 的架构、实现和设计思想，为后续重构工作打下基础。

## 📋 目录

- [一、整体架构理解](#一整体架构理解)
- [二、核心数据流](#二核心数据流)
- [三、学习路径](#三学习路径)
- [四、关键文件清单](#四关键文件清单)
- [五、MVP 设计建议](#五mvp-设计建议)
- [六、实践练习](#六实践练习)

---

## 一、整体架构理解

### 1.1 三层架构

Wren Engine 采用三层架构设计：

```
┌─────────────────────────────────────────────────────────┐
│  ibis-server (Python FastAPI)                          │
│  - Web 服务层                                           │
│  - REST API (v2/v3)                                     │
│  - 路由、中间件、错误处理                                │
│  - 连接器管理 (Connector)                                │
└─────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────┐
│  wren-core (Rust)                                       │
│  - 语义引擎核心                                          │
│  - SQL 规划 (基于 Apache DataFusion)                     │
│  - MDL 分析和转换                                       │
│  - 访问控制 (Row-Level、Column-Level)                   │
└─────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────┐
│  wren-core-legacy (Java)                                │
│  - 遗留引擎 (功能完整但性能较低)                          │
│  - 作为 fallback 使用                                    │
└─────────────────────────────────────────────────────────┘
```

### 1.2 核心组件

| 组件 | 位置 | 作用 | 技术栈 |
|------|------|------|--------|
| **ibis-server** | `ibis-server/` | Web API 服务 | Python, FastAPI, Ibis |
| **wren-core** | `wren-core/` | SQL 规划引擎 | Rust, Apache DataFusion |
| **wren-core-py** | `wren-core-py/` | Python 绑定 | Rust (PyO3) |
| **wren-core-legacy** | `wren-core-legacy/` | Java 引擎 | Java, Trino Parser |

### 1.3 关键概念

#### MDL (Model Definition Language)

MDL 是语义层的核心，用于定义业务模型。示例：

```json
{
  "catalog": "wren",
  "schema": "public",
  "models": [
    {
      "name": "orders",
      "tableReference": {
        "catalog": "jaffle_shop",
        "schema": "main",
        "table": "orders"
      },
      "columns": [
        {
          "name": "amount",
          "type": "double",
          "properties": {
            "description": "Total amount (AUD) of the order"
          }
        }
      ],
      "rowLevelAccessControls": [
        {
          "name": "status_rule",
          "requiredProperties": [
            {
              "name": "session_status",
              "required": false
            }
          ],
          "condition": "status = @session_status"
        }
      ]
    }
  ]
}
```

**参考文件**: `ibis-server/resources/demo/jaffle_shop_mdl.json`

#### 语义层 (Semantic Layer)

语义层将业务术语映射到数据库表，允许用户使用业务语言编写 SQL：

- **输入**: `SELECT * FROM orders_model` (语义 SQL)
- **输出**: `SELECT * FROM jaffle_shop.main.orders WHERE ...` (实际 SQL)

**文档**: https://docs.getwren.ai/oss/engine/concept/what_is_semantics

---

## 二、核心数据流

### 2.1 完整请求流程

```
用户请求
  ↓
POST /v3/connector/postgres/query
  {
    "sql": "SELECT * FROM orders_model",
    "manifestStr": "{...}",
    "connectionInfo": {...}
  }
  ↓
QueryDTO 验证
  ↓
Rewriter.rewrite()
  ├─ experiment=True → EmbeddedEngineRewriter (Rust)
  │   └─ wren_core.SessionContext.transform_sql()
  │       ├─ 1. SQL 解析 (DataFusion Parser)
  │       ├─ 2. 分析规则 (Analyzer Rules)
  │       │   ├─ ExpandWrenViewRule (视图展开)
  │       │   ├─ ModelAnalyzeRule (模型分析)
  │       │   ├─ ModelGenerationRule (模型生成)
  │       │   └─ RelationChain (关系链处理)
  │       ├─ 3. 优化规则 (Optimizer Rules)
  │       │   ├─ WrenTypeCoercion (类型转换)
  │       │   └─ TimestampSimplify (时间戳简化)
  │       └─ 4. SQL 反解析 (WrenDialect)
  │
  └─ experiment=False → ExternalEngineRewriter (Java)
      └─ JavaEngineConnector.dry_plan()
          └─ HTTP 请求 → Java Engine
  ↓
SQL 转换 (sqlglot.transpile)
  ├─ read: Wren (Rust) 或 Trino (Java)
  └─ write: postgres/mysql/snowflake 等
  ↓
Connector.query()
  └─ 执行 SQL → 返回 Arrow Table
  ↓
响应
  {
    "data": [...],
    "columns": [...]
  }
```

### 2.2 关键类和方法

#### Rewriter 类

**位置**: `ibis-server/app/mdl/rewriter.py`

**核心方法**:
```python
async def rewrite(self, sql: str) -> str:
    # 1. 提取 Manifest（可选，用于优化）
    manifest_str = self._extract_manifest(self.manifest_str, sql)

    # 2. SQL 规划（Rust 或 Java）
    planned_sql = await self._rewriter.rewrite(manifest_str, sql, self.properties)

    # 3. SQL 转换（方言转换）
    dialect_sql = self._transpile(planned_sql) if self.data_source else planned_sql

    return dialect_sql
```

#### wren-core transform_sql

**位置**: `wren-core/core/src/mdl/mod.rs`

**核心流程**:
```rust
pub async fn transform_sql_with_ctx(
    ctx: &SessionContext,
    analyzed_mdl: Arc<AnalyzedWrenMDL>,
    remote_functions: &[RemoteFunction],
    properties: SessionPropertiesRef,
    sql: &str,
) -> Result<String> {
    // 1. 注册远程函数
    register_remote_function(ctx, remote_function)?;

    // 2. 应用 Wren 规则到 Context
    let ctx = apply_wren_on_ctx(ctx, analyzed_mdl, properties, Mode::Unparse).await?;

    // 3. 创建逻辑计划
    let plan = ctx.state().create_logical_plan(sql).await?;

    // 4. 优化计划
    let analyzed = ctx.state().optimize(&plan)?;

    // 5. SQL 反解析
    let unparser = Unparser::new(&wren_dialect).with_pretty(true);
    unparser.plan_to_sql(&analyzed)
}
```

---

## 三、学习路径

### 阶段 1: 理解核心概念 (1-2 天)

#### 1.1 MDL 结构

**任务**:
- 阅读 `ibis-server/resources/demo/jaffle_shop_mdl.json`
- 理解 Models、Relationships、Metrics、Views 的结构
- 理解访问控制（Row-Level、Column-Level）

**关键字段**:
- `models`: 业务模型定义
- `relationships`: 模型间关系
- `metrics`: 业务指标
- `views`: 视图定义
- `rowLevelAccessControls`: 行级访问控制
- `columnLevelAccessControl`: 列级访问控制

#### 1.2 语义层概念

**文档**:
- https://docs.getwren.ai/oss/engine/concept/what_is_semantics
- https://docs.getwren.ai/oss/engine/concept/what_is_mdl

**理解要点**:
- 语义层的作用：将业务术语映射到数据库表
- MDL 的作用：定义语义模型
- SQL 规划的作用：将语义 SQL 转换为实际 SQL

#### 1.3 API 接口

**位置**: `ibis-server/app/routers/v3/connector.py`

**核心接口**:
- `POST /v3/connector/{data_source}/query` - 执行查询
- `POST /v3/connector/{data_source}/dry-plan` - SQL 规划（不执行）
- `GET /v3/connector/{data_source}/functions` - 获取函数列表

**请求示例**:
```python
POST /v3/connector/postgres/query
{
    "sql": "SELECT * FROM orders_model",
    "manifestStr": "base64_encoded_mdl_json",
    "connectionInfo": {
        "host": "localhost",
        "port": 5432,
        "database": "test",
        "user": "postgres",
        "password": "password"
    }
}
```

---

### 阶段 2: 理解 SQL 规划流程 (2-3 天)

#### 2.1 Rewriter 类

**位置**: `ibis-server/app/mdl/rewriter.py`

**学习重点**:
1. `Rewriter.__init__()` - 如何选择引擎（Rust/Java）
2. `Rewriter.rewrite()` - SQL 规划流程
3. `Rewriter._transpile()` - SQL 方言转换
4. `Rewriter._extract_manifest()` - Manifest 提取优化

**关键代码**:
```python
class Rewriter:
    def __init__(self, manifest_str, data_source, experiment=False):
        if experiment:
            # 使用 Rust 引擎
            self._rewriter = EmbeddedEngineRewriter(function_path)
        else:
            # 使用 Java 引擎
            self._rewriter = ExternalEngineRewriter(java_engine_connector)
```

#### 2.2 wren-core 的 transform_sql

**位置**: `wren-core/core/src/mdl/mod.rs`

**学习重点**:
1. `transform_sql_with_ctx()` - 完整的 SQL 转换流程
2. `apply_wren_on_ctx()` - 如何应用 Wren 规则
3. `create_logical_plan()` - 逻辑计划创建
4. `optimize()` - 计划优化

**实践**:
- 运行 `wren-core/wren-example/examples/plan-sql.rs`
- 观察输入 SQL 和输出 SQL 的差异

#### 2.3 分析规则 (Analyzer Rules)

**位置**: `wren-core/core/src/logical_plan/analyze/`

**规则列表**:
- `expand_view.rs` - 视图展开
- `model_anlayze.rs` - 模型分析
- `model_generation.rs` - 模型生成
- `relation_chain.rs` - 关系链处理
- `access_control.rs` - 访问控制

**学习重点**:
- 每个规则的作用
- 规则如何修改逻辑计划
- 规则的执行顺序

#### 2.4 优化规则 (Optimizer Rules)

**位置**: `wren-core/core/src/logical_plan/optimize/`

**规则列表**:
- `type_coercion.rs` - 类型转换
- `simplify_timestamp.rs` - 时间戳简化

**学习重点**:
- 优化规则的作用
- 如何优化查询性能

---

### 阶段 3: 理解数据源连接 (1-2 天)

#### 3.1 Connector 类

**位置**: `ibis-server/app/model/connector.py`

**学习重点**:
1. `Connector.__init__()` - 如何选择连接器
2. `Connector.query()` - 如何执行 SQL
3. `Connector.dry_run()` - 如何验证 SQL

**支持的连接器**:
- `PostgresConnector` - PostgreSQL
- `MSSqlConnector` - SQL Server
- `BigQueryConnector` - BigQuery
- `DuckDBConnector` - DuckDB (本地文件)
- `SimpleConnector` - 其他数据库

#### 3.2 连接器实现

**学习重点**:
- 如何使用 Ibis 框架连接数据库
- 如何执行 SQL 查询
- 如何返回 Arrow Table 格式

**实践**:
- 阅读 `PostgresConnector` 的实现
- 理解如何连接 PostgreSQL 并执行查询

---

### 阶段 4: 运行和测试 (1 天)

#### 4.1 运行示例

**步骤**:
```bash
# 1. 进入 ibis-server 目录
cd wren-engine/ibis-server

# 2. 安装依赖
poetry install

# 3. 运行服务
poetry run python -m app.main
```

**验证**:
- 访问 http://localhost:8080/docs 查看 API 文档
- 访问 http://localhost:8080/health 检查健康状态

#### 4.2 运行测试用例

**位置**: `ibis-server/tests/routers/v3/connector/postgres/`

**运行测试**:
```bash
# 运行 PostgreSQL 测试
pytest tests/routers/v3/connector/postgres/test_query.py -v
```

**学习重点**:
- 测试用例如何准备 MDL
- 测试用例如何发送请求
- 测试用例如何验证结果

#### 4.3 理解测试流程

**典型测试流程**:
1. 准备测试数据（MDL、数据库）
2. 发送查询请求
3. 验证返回结果
4. 清理测试数据

---

## 四、关键文件清单

### 4.1 必须阅读的文件

| 文件 | 说明 | 优先级 |
|------|------|--------|
| `API_ROUTING.md` | API 路由机制和引擎选择 | ⭐⭐⭐ |
| `MIGRATION_STATUS.md` | 功能完成度和迁移状态 | ⭐⭐⭐ |
| `ibis-server/app/routers/v3/connector.py` | v3 API 实现 | ⭐⭐⭐ |
| `ibis-server/app/mdl/rewriter.py` | SQL 重写逻辑 | ⭐⭐⭐ |
| `ibis-server/app/mdl/core.py` | wren-core Python 绑定 | ⭐⭐⭐ |
| `wren-core/core/src/mdl/mod.rs` | wren-core 核心逻辑 | ⭐⭐⭐ |
| `wren-core/wren-example/examples/plan-sql.rs` | 使用示例 | ⭐⭐ |
| `ibis-server/app/model/connector.py` | 连接器实现 | ⭐⭐ |
| `ibis-server/resources/demo/jaffle_shop_mdl.json` | MDL 示例 | ⭐⭐ |

### 4.2 参考文档

| 文档 | 说明 |
|------|------|
| `README.md` | 项目概述 |
| `REWRITE_PLAN.md` | 重构计划 |
| `wren-core/COMPLETION_STATUS.md` | Rust 版本完成度 |
| `wren-core/README.md` | wren-core 文档 |

### 4.3 测试用例参考

| 文件 | 说明 |
|------|------|
| `ibis-server/tests/routers/v3/connector/postgres/test_query.py` | PostgreSQL 查询测试 |
| `ibis-server/tests/routers/v3/connector/postgres/test_functions.py` | 函数测试 |

---

## 五、MVP 设计建议

### 5.1 MVP 范围

基于当前架构，MVP 应该包含：

#### 必须功能
- ✅ Rust Axum 服务器（替代 FastAPI）
- ✅ 直接使用 wren-core（不需要 Python 绑定）
- ✅ PostgreSQL 连接器
- ✅ 核心 API：`/v3/connector/postgres/query`
- ✅ 核心 API：`/v3/connector/postgres/dry-plan`

#### 可选功能（MVP 可省略）
- ❌ 查询缓存
- ❌ 多数据源支持（先只支持 PostgreSQL）
- ❌ Fallback 机制（先只支持 Rust 引擎）
- ❌ 元数据查询 API

### 5.2 技术栈选择

| 组件 | 当前技术 | MVP 技术 | 说明 |
|------|---------|---------|------|
| Web 框架 | FastAPI (Python) | Axum (Rust) | 性能更好 |
| SQL 规划 | wren-core (Rust) | wren-core (Rust) | 直接使用 |
| SQL 转换 | sqlglot (Python) | sqlglot-rs 或 FFI | 需要评估 |
| 数据库连接 | Ibis (Python) | tokio-postgres (Rust) | 简化实现 |
| 数据格式 | Arrow Table | JSON | MVP 简化 |

### 5.3 项目结构

```
wren-server/
├── Cargo.toml
├── src/
│   ├── main.rs              # Axum 服务器入口
│   ├── api/
│   │   ├── mod.rs
│   │   └── v3/
│   │       └── connector.rs # v3 API 路由
│   ├── engine/
│   │   ├── mod.rs
│   │   └── rewriter.rs      # SQL 规划封装
│   ├── connector/
│   │   ├── mod.rs
│   │   └── postgres.rs      # PostgreSQL 连接器
│   └── model/
│       ├── mod.rs
│       ├── request.rs        # 请求模型
│       └── response.rs       # 响应模型
└── tests/
    └── integration/
        └── test_query.rs     # 集成测试
```

### 5.4 实现步骤

#### Step 1: 创建项目结构
```bash
cargo new wren-server
cd wren-server
```

#### Step 2: 添加依赖
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
wren-core = { path = "../wren-core/core" }
tokio-postgres = "0.7"
```

#### Step 3: 实现核心 API
- 实现 `/v3/connector/postgres/query`
- 实现 `/v3/connector/postgres/dry-plan`

#### Step 4: 集成测试
- 编写集成测试
- 对比原版本输出

---

## 六、实践练习

### 练习 1: 理解 MDL

**任务**: 阅读并理解 `jaffle_shop_mdl.json`

**问题**:
1. MDL 中有哪些模型？
2. 每个模型有哪些字段？
3. 有哪些访问控制规则？

### 练习 2: 运行示例

**任务**: 运行 `wren-core/wren-example/examples/plan-sql.rs`

**步骤**:
```bash
cd wren-engine/wren-core/wren-example
cargo run --example plan-sql
```

**观察**:
- 输入 SQL 是什么？
- 输出 SQL 是什么？
- 发生了什么转换？

### 练习 3: 运行测试

**任务**: 运行 PostgreSQL 测试用例

**步骤**:
```bash
cd wren-engine/ibis-server
pytest tests/routers/v3/connector/postgres/test_query.py -v -k test_simple_query
```

**观察**:
- 测试如何准备数据？
- 测试如何发送请求？
- 测试如何验证结果？

### 练习 4: 跟踪一个请求

**任务**: 跟踪一个完整的查询请求

**步骤**:
1. 启动 ibis-server
2. 发送一个查询请求
3. 在代码中添加日志，跟踪数据流

**观察**:
- 请求经过哪些函数？
- 每个函数做了什么？
- 数据如何转换？

---

## 七、学习时间表

| 阶段 | 内容 | 时间 | 完成标志 |
|------|------|------|----------|
| **阶段 1** | 理解核心概念 | 1-2 天 | 能够解释 MDL 和语义层 |
| **阶段 2** | 理解 SQL 规划 | 2-3 天 | 能够解释 SQL 规划流程 |
| **阶段 3** | 理解数据源连接 | 1-2 天 | 能够解释连接器实现 |
| **阶段 4** | 运行和测试 | 1 天 | 能够运行和调试代码 |
| **阶段 5** | MVP 设计 | 2-3 天 | 完成 MVP 设计文档 |
| **阶段 6** | MVP 实现 | 2-3 周 | 完成 MVP 实现 |

**总计**: 约 3-4 周

---

## 八、常见问题

### Q1: wren-core 和 wren-core-legacy 的区别？

**A**:
- `wren-core` (Rust): 新版本，性能更好，但功能不完整（约 75%）
- `wren-core-legacy` (Java): 旧版本，功能完整，但性能较低

### Q2: 什么时候使用 Rust 引擎，什么时候使用 Java 引擎？

**A**:
- v3 API 默认使用 Rust 引擎
- 如果 Rust 引擎不支持（如 CumulativeMetric），会自动 fallback 到 Java 引擎
- 可以通过 `experiment=False` 强制使用 Java 引擎

### Q3: SQL 规划的具体流程是什么？

**A**:
1. SQL 解析（DataFusion Parser）
2. 分析规则（视图展开、模型分析等）
3. 优化规则（类型转换、时间戳简化等）
4. SQL 反解析（WrenDialect）

### Q4: 如何添加新的数据源？

**A**:
参考 `ibis-server/docs/how-to-add-data-source.md`

---

## 九、参考资料

### 官方文档
- [Wren Engine 文档](https://docs.getwren.ai/oss/engine/)
- [什么是语义层](https://docs.getwren.ai/oss/engine/concept/what_is_semantics)
- [什么是 MDL](https://docs.getwren.ai/oss/engine/concept/what_is_mdl)

### 技术文档
- [Apache DataFusion](https://arrow.apache.org/datafusion/)
- [Axum 文档](https://docs.rs/axum/)
- [Ibis 文档](https://ibis-project.org/)

### 相关文件
- `API_ROUTING.md` - API 路由机制
- `MIGRATION_STATUS.md` - 迁移状态
- `REWRITE_PLAN.md` - 重构计划

---

## 十、下一步行动

1. ✅ 阅读本文档
2. ⬜ 完成阶段 1: 理解核心概念
3. ⬜ 完成阶段 2: 理解 SQL 规划
4. ⬜ 完成阶段 3: 理解数据源连接
5. ⬜ 完成阶段 4: 运行和测试
6. ⬜ 设计 MVP
7. ⬜ 实现 MVP

---

**最后更新**: 2024-12-19
**维护者**: Wren Engine Team
