# Marrow

[English](README.md)

`Marrow` 是一个为多样化设备设计的通用运行时， 当前 `Marrow` 使用 `Webassembly` 作为运行时来进行实现。

## 设计

`Marrow` 允许同时加载多个 `Webassembly` 模块，这些模块之间通过CABI进行相互调用。同时 `Marrow` 提供了多个语言可用的标准库 `mw-std` ，这些标准库实现在运行时中，可以实现对模块所运行的节点资源进行访问。

未来 `Marrow` 支持将各个模块以分布式的方式在各个节点上进行使用。最终 `Marrow` 的实现结构如下：

![design](docs/assets/design.png)

### ABI调用管理器

ABI调用管理器负责各个模块之间，与

### API拦截器

## 版本计划

### 单节点 Javascript 版本

> 当前版本

单节点 Javascript 版本可以使用

### 单节点 Rust 版本

### 分布式实现
