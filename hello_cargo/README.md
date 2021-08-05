## 使用cargo创建项目
cargo new hellp_cargo

- 项目名称是hello_cargo
- 会创建新的目录hello_cargo
  - Cargo.toml
  - src
    - main.rs
  - 初始化了git仓库, .gitignore
    - 可以使用其他vcs, 或者不使用vcs, --vcs 这个flag 

## Cargo.toml
- TOML (Tom's Obvious, Minimal Language)
- [package], 是一个区域的标题, 表示下方内容是package的配置
  - name
  - version
  - authors
  - edition
- [dependency]
- 在rust中, 他的代码被称作 crate

## 规范
- Cargo.toml以及其他和代码无关的文件放在顶层目录
- 源代码都应该在src
- 创建项目没有使用cargo, 也可以把项目转化为cargo
  - 源代码移到src
  - 创建Cargo.toml

## build
- cargo build
  - 创建可执行文件: target/debug/*
- 第一次build会在顶层目录生成cargo.lock文件
  - 负责追踪项目依赖的精确版本
  - 不要手动修改

## build & run
- cargo run 编译 + 执行结果
  - 增量编译(编译成功过, 且没有修改过源代码, 直接执行二进制文件)

## check
- cargo check, 检查代码, 确保能通过编译, 但是不产生文件
- cargo check 比 cargo build 快得多
  - 用于在编写代码的时候连续反复检查代码, 提高效率

## release
- cargo build --release
  - 编译时优化
    - 代码运行更快, 但是编译时间更长
  - 产物在target/release
- 两种配置
  - 开发
  - 正式发布


## 尽量使用cargo