# mimir_well_engine



## 作者

- **binghu <hqf0330@gamil.com>**

## 项目信息

- **仓库**: 
- **许可证**: 
- **Rust 版本**: 2021 Edition

## 快速开始

### 安装依赖

确保你已经安装了 Rust 工具链：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 构建项目

```bash
cargo build
```

### 运行项目

```bash
cargo run
```

### 运行测试

```bash
cargo test
```

## 开发工具

这个项目配置了以下开发工具来保证代码质量：

### 必需工具

```bash
# 安装 cargo-generate (用于生成新项目)
cargo install cargo-generate

# 安装 pre-commit (代码提交前检查)
pipx install pre-commit

# 安装 cargo-deny (依赖安全检查)
cargo install --locked cargo-deny

# 安装 typos (拼写检查)
cargo install typos-cli

# 安装 git-cliff (自动生成 changelog)
cargo install git-cliff

# 安装 cargo-nextest (增强测试工具)
cargo install cargo-nextest --locked
```

### 初始化开发环境

```bash
# 安装 pre-commit hooks
pre-commit install

# 运行所有检查
cargo deny check
typos
cargo test
```

## 使用模板生成新项目

```bash
cargo generate hqf0330/template
```

## VSCode 推荐插件

- rust-analyzer: Rust 语言支持
- crates: Rust 包管理
- Even Better TOML: TOML 文件支持
- Error Lens: 错误提示优化
- GitLens: Git 增强
- Better Comments: 优化注释显示
- indent-rainbow: 缩进显示优化
- TODO Highlight: TODO 高亮
- vscode-icons: 图标优化

## 许可证

本项目采用  许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。
