# Wren Engine 代码阅读学习顺序指南

> 本文档提供系统性的代码阅读顺序，帮助开发者从入门到深入理解 wren-engine 的实现。

## 📋 目录

- [一、准备阶段](#一准备阶段)
- [二、基础层：数据结构](#二基础层数据结构)
- [三、核心层：SQL 规划](#三核心层sql-规划)
- [四、应用层：API 服务](#四应用层api-服务)
- [五、实践层：示例和测试](#五实践层示例和测试)
- [六、学习时间表](#六学习时间表)

---

## 一、准备阶段

### 1.1 理解整体架构（30 分钟）

**必读文档**：
1. `README.md` - 项目概述
2. `LEARNING_GUIDE.md` - 学习规划文档
3. `API_ROUTING.md` - API 路由机制
4. `MIGRATION_STATUS.md` - 功能完成度

**目标**：
- 理解三层架构：ibis-server (Python) → wren-core (Rust) → wren-core-legacy (Java)
- 理解核心概念：MDL、语义层、SQL 规划
- 理解数据流：请求 → SQL 规划 → SQL 转换 → 执行

---

## 二、基础层：数据结构

### 阶段 2.1：MDL 数据结构（1-2 天）

**目标**：理解 MDL 的数据结构定义

#### 2.1.1 wren-core-base（基础数据结构）

**位置**：`wren-core-base/src/mdl/`

**阅读顺序**：

1. **`manifest.rs`** ⭐⭐⭐
   - 理解 `Manifest` 结构
   - 理解 `Model`、`Column`、`Relationship`、`Metric`、`View` 的定义
   - 理解访问控制结构（`RowLevelAccessControl`、`ColumnLevelAccessControl`）

2. **`builder.rs`** ⭐⭐
   - 理解如何构建 MDL
   - `ManifestBuilder`、`ModelBuilder`、`ColumnBuilder` 等
   - 学习构建器模式的使用

**关键代码**：
```rust
// wren-core-base/src/mdl/manifest.rs
pub struct Manifest {
    pub catalog: String,
    pub schema: String,
    pub models: Vec<Arc<Model>>,
    pub relationships: Vec<Arc<Relationship>>,
    // ...
}
```

**实践**：
- 阅读 `wren-core-base/tests/data/mdl.json` 理解 MDL 格式
- 尝试用 `ManifestBuilder` 构建一个简单的 MDL

---

### 阶段 2.2：理解 MDL 分析（1 天）

**位置**：`wren-core/core/src/mdl/`

#### 2.2.1 `mod.rs` ⭐⭐⭐

**阅读重点**：
- `AnalyzedWrenMDL` 结构
- `WrenMDL` 结构
- `analyze()` 方法如何分析 MDL

**关键代码**：
```rust
// wren-core/core/src/mdl/mod.rs
pub struct AnalyzedWrenMDL {
    pub wren_mdl: Arc<WrenMDL>,
    pub lineage: Arc<lineage::Lineage>,
}

impl AnalyzedWrenMDL {
    pub fn analyze(...) -> Result<Self> {
        // 分析 MDL，生成 WrenMDL
    }
}
```

#### 2.2.2 `lineage.rs` ⭐⭐

**阅读重点**：
- 数据血缘分析
- 理解字段依赖关系

#### 2.2.3 `context.rs` ⭐⭐⭐

**阅读重点**：
- `apply_wren_on_ctx()` - 如何将 MDL 应用到 DataFusion Context
- `Mode` 枚举（LocalRuntime、Unparse、PermissionAnalyze）
- 理解 SessionContext 的配置

**关键代码**：
```rust
// wren-core/core/src/mdl/context.rs
pub async fn apply_wren_on_ctx(
    ctx: &SessionContext,
    analyzed_mdl: Arc<AnalyzedWrenMDL>,
    properties: SessionPropertiesRef,
    mode: Mode,
) -> Result<SessionContext> {
    // 应用 Wren 规则到 Context
}
```

---

## 三、核心层：SQL 规划

### 阶段 3.1：SQL 转换入口（1 天）

**位置**：`wren-core/core/src/mdl/mod.rs`

#### 3.1.1 `transform_sql_with_ctx()` ⭐⭐⭐

**阅读重点**：
- 完整的 SQL 转换流程
- 理解输入输出
- 理解错误处理

**关键代码**：
```rust
// wren-core/core/src/mdl/mod.rs
pub async fn transform_sql_with_ctx(
    ctx: &SessionContext,
    analyzed_mdl: Arc<AnalyzedWrenMDL>,
    remote_functions: &[RemoteFunction],
    properties: SessionPropertiesRef,
    sql: &str,
) -> Result<String> {
    // 1. 注册远程函数
    // 2. 应用 Wren 规则
    // 3. 创建逻辑计划
    // 4. 优化计划
    // 5. SQL 反解析
}
```

**实践**：
- 运行 `wren-example/examples/plan-sql.rs` 观察输入输出
- 单步调试理解流程

---

### 阶段 3.2：分析规则（Analyzer Rules）（2-3 天）

**位置**：`wren-core/core/src/logical_plan/analyze/`

#### 3.2.1 `mod.rs` ⭐⭐

**阅读重点**：
- 理解分析规则的注册
- 理解规则的执行顺序

#### 3.2.2 `expand_view.rs` ⭐⭐⭐

**阅读重点**：
- 视图展开逻辑
- 理解如何将视图替换为实际 SQL

**关键概念**：
- `ExpandWrenViewRule` - 视图展开规则
- 如何递归展开嵌套视图

#### 3.2.3 `model_anlayze.rs` ⭐⭐⭐

**阅读重点**：
- 模型分析逻辑
- 理解如何分析 SQL 中使用的模型

#### 3.2.4 `model_generation.rs` ⭐⭐⭐

**阅读重点**：
- 模型生成逻辑
- 理解如何将语义模型转换为实际表

**关键代码**：
```rust
// wren-core/core/src/logical_plan/analyze/model_generation.rs
pub struct ModelGenerationRule {
    // 将 orders_model 转换为 jaffle_shop.main.orders
}
```

#### 3.2.5 `relation_chain.rs` ⭐⭐⭐

**阅读重点**：
- 关系链处理
- 理解如何自动添加 JOIN

**关键概念**：
- `RelationChain` - 关系链分析
- 如何根据关系自动生成 JOIN 条件

#### 3.2.6 `access_control.rs` ⭐⭐⭐

**阅读重点**：
- 访问控制规则
- 行级访问控制（Row-Level）
- 列级访问控制（Column-Level）

**关键代码**：
```rust
// wren-core/core/src/logical_plan/analyze/access_control.rs
pub struct AccessControlRule {
    // 自动添加 WHERE 和 SELECT 过滤
}
```

---

### 阶段 3.3：优化规则（Optimizer Rules）（1-2 天）

**位置**：`wren-core/core/src/logical_plan/optimize/`

#### 3.3.1 `type_coercion.rs` ⭐⭐

**阅读重点**：
- 类型转换优化
- 理解如何自动转换数据类型

#### 3.3.2 `simplify_timestamp.rs` ⭐⭐

**阅读重点**：
- 时间戳简化
- 理解如何优化时间戳表达式

---

### 阶段 3.4：SQL 反解析（1 天）

**位置**：`wren-core/core/src/mdl/dialect/`

#### 3.4.1 `wren_dialect.rs` ⭐⭐⭐

**阅读重点**：
- WrenDialect 定义
- 理解如何将逻辑计划转换为 SQL

#### 3.4.2 `mod.rs` ⭐⭐

**阅读重点**：
- 方言管理
- 理解如何支持不同数据库方言

---

## 四、应用层：API 服务

### 阶段 4.1：Python 绑定（可选，1 天）

**位置**：`wren-core-py/src/`

**阅读重点**：
- `context.rs` - Python 绑定的 SessionContext
- `manifest.rs` - Python 绑定的 Manifest
- 理解 PyO3 的使用

**注意**：如果只做 Rust 重构，可以跳过这部分

---

### 阶段 4.2：ibis-server（当前实现，参考用）

**位置**：`ibis-server/app/`

#### 4.2.1 `main.py` ⭐⭐

**阅读重点**：
- FastAPI 应用入口
- 理解服务启动流程

#### 4.2.2 `routers/v3/connector.py` ⭐⭐⭐

**阅读重点**：
- v3 API 实现
- `query()` 和 `dry-plan()` 接口
- 理解如何调用 wren-core

**关键代码**：
```python
# ibis-server/app/routers/v3/connector.py
async def query(...):
    # 1. 创建 Rewriter
    rewritten_sql = await Rewriter(
        dto.manifest_str,
        data_source=data_source,
        experiment=True,  # 使用 Rust 引擎
    ).rewrite(sql)

    # 2. 执行查询
    result = await execute_query_with_timeout(...)
```

#### 4.2.3 `mdl/rewriter.py` ⭐⭐⭐

**阅读重点**：
- `Rewriter` 类
- `EmbeddedEngineRewriter` (Rust)
- `ExternalEngineRewriter` (Java)
- SQL 转换流程

**关键代码**：
```python
# ibis-server/app/mdl/rewriter.py
class Rewriter:
    async def rewrite(self, sql: str) -> str:
        # 1. 提取 Manifest
        # 2. SQL 规划
        # 3. SQL 转换（方言转换）
```

#### 4.2.4 `mdl/core.py` ⭐⭐

**阅读重点**：
- wren-core Python 绑定调用
- `get_session_context()` 和 `transform_sql()`

#### 4.2.5 `model/connector.py` ⭐⭐

**阅读重点**：
- `Connector` 类
- 理解如何连接不同数据源
- 理解如何执行 SQL

**注意**：这部分是当前 Python 实现，重构时会用 Rust 替代

---

## 五、实践层：示例和测试

### 阶段 5.1：运行示例（1 天）

**位置**：`wren-core/wren-example/examples/`

#### 5.1.1 `plan-sql.rs` ⭐⭐⭐

**阅读重点**：
- 最简单的 SQL 规划示例
- 理解如何使用 `transform_sql_with_ctx()`

**运行**：
```bash
cd wren-engine/wren-core/wren-example
cargo run --example plan-sql
```

#### 5.1.2 `view.rs` ⭐⭐

**阅读重点**：
- 视图展开示例
- 理解视图如何工作

#### 5.1.3 `row-level-access-control.rs` ⭐⭐

**阅读重点**：
- 访问控制示例
- 理解访问控制如何工作

#### 5.1.4 其他示例

- `datafusion-apply.rs` - DataFusion 集成
- `calculation-invoke-calculation.rs` - 计算字段
- `to-many-calculation.rs` - 一对多计算

---

### 阶段 5.2：运行测试（1 天）

**位置**：`wren-core/core/tests/` 和 `wren-core/sqllogictest/`

#### 5.2.1 单元测试

**位置**：`wren-core/core/src/mdl/mod.rs`（测试在文件末尾）

**运行**：
```bash
cd wren-engine/wren-core/core
cargo test
```

#### 5.2.2 SQL Logic Test

**位置**：`wren-core/sqllogictest/`

**运行**：
```bash
cd wren-engine/wren-core/sqllogictest
cargo test
```

**理解**：
- SQL Logic Test 格式
- 如何编写测试用例

---

### 阶段 5.3：运行基准测试（可选，1 天）

**位置**：`wren-core/benchmarks/`

**运行**：
```bash
cd wren-engine/wren-core/benchmarks
./bench.sh run tpch
./bench.sh run wren
```

**理解**：
- 性能测试方法
- 如何对比性能

---

## 六、学习时间表

### 第一周：基础理解

| 天数 | 内容 | 时间 |
|------|------|------|
| Day 1 | 准备阶段：阅读文档 | 4 小时 |
| Day 2-3 | 阶段 2.1：MDL 数据结构 | 8 小时 |
| Day 4 | 阶段 2.2：MDL 分析 | 4 小时 |
| Day 5 | 阶段 3.1：SQL 转换入口 | 4 小时 |

### 第二周：核心功能

| 天数 | 内容 | 时间 |
|------|------|------|
| Day 6-8 | 阶段 3.2：分析规则 | 12 小时 |
| Day 9 | 阶段 3.3：优化规则 | 4 小时 |
| Day 10 | 阶段 3.4：SQL 反解析 | 4 小时 |

### 第三周：应用和实践

| 天数 | 内容 | 时间 |
|------|------|------|
| Day 11 | 阶段 4.2：ibis-server（参考） | 4 小时 |
| Day 12-13 | 阶段 5.1：运行示例 | 8 小时 |
| Day 14 | 阶段 5.2：运行测试 | 4 小时 |

**总计**：约 3 周（60 小时）

---

## 七、关键文件优先级

### ⭐⭐⭐ 必须深入理解

1. `wren-core-base/src/mdl/manifest.rs` - MDL 数据结构
2. `wren-core/core/src/mdl/mod.rs` - 核心转换逻辑
3. `wren-core/core/src/mdl/context.rs` - Context 应用
4. `wren-core/core/src/logical_plan/analyze/model_generation.rs` - 模型生成
5. `wren-core/core/src/logical_plan/analyze/relation_chain.rs` - 关系链
6. `wren-core/core/src/logical_plan/analyze/access_control.rs` - 访问控制
7. `ibis-server/app/mdl/rewriter.py` - SQL 重写（参考）

### ⭐⭐ 重要理解

1. `wren-core-base/src/mdl/builder.rs` - MDL 构建器
2. `wren-core/core/src/mdl/lineage.rs` - 数据血缘
3. `wren-core/core/src/logical_plan/analyze/expand_view.rs` - 视图展开
4. `wren-core/core/src/logical_plan/analyze/model_anlayze.rs` - 模型分析
5. `wren-core/core/src/mdl/dialect/wren_dialect.rs` - SQL 反解析
6. `ibis-server/app/routers/v3/connector.py` - API 实现（参考）

### ⭐ 了解即可

1. `wren-core/core/src/logical_plan/optimize/` - 优化规则
2. `wren-core/core/src/mdl/function/` - 函数系统
3. `ibis-server/app/model/connector.py` - 连接器（参考）

---

## 八、学习技巧

### 1. 从示例开始

**推荐顺序**：
1. 先运行 `plan-sql.rs` 示例，观察输入输出
2. 单步调试，理解代码执行流程
3. 然后阅读相关源代码

### 2. 理解数据流

**关键流程**：
```
用户 SQL
  ↓
transform_sql_with_ctx()
  ↓
apply_wren_on_ctx() - 应用 MDL 到 Context
  ↓
create_logical_plan() - 创建逻辑计划
  ↓
分析规则（Analyzer Rules）
  - ExpandWrenViewRule
  - ModelAnalyzeRule
  - ModelGenerationRule
  - RelationChain
  - AccessControlRule
  ↓
优化规则（Optimizer Rules）
  - WrenTypeCoercion
  - TimestampSimplify
  ↓
unparser.plan_to_sql() - SQL 反解析
  ↓
最终 SQL
```

### 3. 对比理解

**推荐**：
- 对比输入 SQL 和输出 SQL
- 理解每个规则做了什么转换
- 使用 `println!` 或日志观察中间结果

### 4. 阅读测试用例

**位置**：
- `wren-core/core/src/mdl/mod.rs`（测试在文件末尾）
- `wren-core/sqllogictest/test_files/`

**好处**：
- 理解功能的使用方式
- 理解边界情况

---

## 九、实践建议

### 1. 边读边运行

**不要只读代码**：
- 运行示例代码
- 修改示例代码，观察变化
- 单步调试理解流程

### 2. 画流程图

**建议**：
- 画出 SQL 转换的流程图
- 画出每个分析规则的作用
- 画出数据结构的层次关系

### 3. 写注释

**建议**：
- 在关键代码处写注释
- 总结每个函数的作用
- 记录自己的理解

### 4. 提问和验证

**建议**：
- 对不理解的地方提问
- 通过运行代码验证理解
- 对比文档和代码

---

## 十、常见问题

### Q1: 应该先读 Rust 代码还是 Python 代码？

**A**:
- 如果要做 Rust 重构，**优先读 Rust 代码**（wren-core）
- Python 代码（ibis-server）作为参考，理解接口和流程
- 建议顺序：Rust 核心 → Python 接口 → Rust 示例

### Q2: 如何理解分析规则？

**A**:
1. 先理解每个规则的作用（看注释和文档）
2. 运行示例，观察规则的效果
3. 单步调试，看规则如何修改逻辑计划
4. 对比输入输出 SQL

### Q3: 如何理解 MDL 到 SQL 的转换？

**A**:
1. 先理解 MDL 结构（manifest.rs）
2. 理解如何将 MDL 应用到 Context（context.rs）
3. 理解分析规则如何转换（analyze/）
4. 理解 SQL 反解析（dialect/）

### Q4: 如何验证理解是否正确？

**A**:
1. 运行示例代码
2. 运行测试用例
3. 修改代码，观察变化
4. 对比文档和实现

---

## 十一、下一步行动

1. ✅ 阅读本文档
2. ⬜ 完成准备阶段：阅读文档
3. ⬜ 完成阶段 2：理解数据结构
4. ⬜ 完成阶段 3：理解 SQL 规划
5. ⬜ 完成阶段 5：运行示例和测试
6. ⬜ 开始 MVP 开发

---

**最后更新**: 2024-12-19
**维护者**: Wren Engine Team
