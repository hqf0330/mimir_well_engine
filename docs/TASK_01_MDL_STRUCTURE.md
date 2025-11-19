# 学习任务 01: MDL 数据结构理解与实现

> **目标**: 理解 MDL (Model Definition Language) 的数据结构，并实现基础的 MDL 解析和验证

## 📋 任务目标

1. **理解 MDL 的核心数据结构**
   - Manifest（清单）
   - Model（模型）
   - Column（列）
   - Relationship（关系）

2. **实现基础功能**
   - MDL JSON 解析（从 base64 字符串解析）
   - MDL 结构验证（必填字段检查）
   - 简单的错误处理

## 🎯 学习重点

### 1. MDL 结构理解

参考 `wren-engine/wren-core-base/src/mdl/manifest.rs`，理解以下结构：

```rust
pub struct Manifest {
    pub catalog: String,        // 目录名
    pub schema: String,          // Schema 名
    pub models: Vec<Model>,      // 模型列表
    pub relationships: Vec<Relationship>, // 关系列表
}

pub struct Model {
    pub name: String,                    // 模型名称
    pub table_reference: TableReference, // 表引用
    pub columns: Vec<Column>,            // 列定义
}

pub struct TableReference {
    pub catalog: String,
    pub schema: String,
    pub table: String,
}

pub struct Column {
    pub name: String,  // 列名
    pub r#type: String, // 列类型
}

pub struct Relationship {
    pub name: String,        // 关系名称
    pub models: Vec<String>, // 关联的模型
    pub join_type: String,   // JOIN 类型
    pub condition: String,   // JOIN 条件
}
```

### 2. 需要实现的功能

#### 2.1 MDL 解析 (`src/mdl/analyzer.rs`)

```rust
impl Analyzer {
    /// 从 base64 编码的 JSON 字符串解析 MDL
    pub fn parse_manifest(manifest_str: &str) -> Result<Manifest> {
        // TODO:
        // 1. base64 解码
        // 2. JSON 反序列化
        // 3. 返回 Manifest
    }
}
```

#### 2.2 MDL 验证 (`src/mdl/analyzer.rs`)

```rust
impl Analyzer {
    /// 验证 MDL 结构
    pub fn validate(&self, manifest: &Manifest) -> Result<()> {
        // TODO:
        // 1. 检查 catalog 和 schema 是否为空
        // 2. 检查 models 是否为空
        // 3. 检查每个 model 的必填字段
        // 4. 检查 relationships 的有效性
    }
}
```

## 📝 实现步骤

### Step 1: 完善 `src/model/manifest.rs`

确保所有结构体都实现了 `Serialize` 和 `Deserialize`，并且字段完整。

### Step 2: 实现 `src/mdl/analyzer.rs`

1. 添加 `parse_manifest()` 方法
   - 使用 `base64` crate 解码
   - 使用 `serde_json` 反序列化

2. 添加 `validate()` 方法
   - 验证必填字段
   - 返回清晰的错误信息

### Step 3: 编写测试

在 `src/mdl/analyzer.rs` 中添加测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_manifest() {
        // 测试解析功能
    }

    #[test]
    fn test_validate_manifest() {
        // 测试验证功能
    }
}
```

## 📚 参考资源

1. **代码参考**:
   - `wren-engine/wren-core-base/src/mdl/manifest.rs` - MDL 结构定义
   - `wren-engine/wren-core-base/tests/data/mdl.json` - MDL 示例

2. **文档参考**:
   - `docs/CODE_READING_GUIDE.md` - 阶段 2.1
   - `docs/LEARNING_GUIDE.md` - MDL 概念说明

## ✅ 完成标准

- [ ] 能够从 base64 字符串解析 MDL JSON
- [ ] 能够验证 MDL 的基本结构（必填字段）
- [ ] 有清晰的错误提示
- [ ] 通过单元测试

## 🚀 下一步

完成此任务后，下一步是：**任务 02 - MDL 分析器实现**（理解 `AnalyzedWrenMDL` 和 `Lineage`）

---

**预计时间**: 2-3 小时
**难度**: ⭐⭐ (中等)
