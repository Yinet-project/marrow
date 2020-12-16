# Marrow
Marrow 是一个安全的运行时环境，Marrow 提供了一套层次化的API体系，以及对使用API访问外部环境时的控制。Marrow允许将 Webassembly 环境应用至包括嵌入式设备在内的多种设备环境中。

## Roadmap

- [ ] Marrow
  - [X] NativeModule.
  - [X] WasmModule.
  - [X] Runtime.
  - [X] Multimodule support.
  - [X] Target for (win, mac, linux).
  - [ ] `Future`/`Promise` for `cabi`.
  - [ ] API Manager.
  - [ ] I/O library for unversal platform.
  - [ ] Network library for unversal platform.
  - [ ] Target for web.
  - [ ] Target for embedded.
- [ ] Testing.
  - [X] Test alloc.
  - [ ] Test wasmi in wasmi.
  - [ ] Test wasmi in embedded system.
- [ ] Actor framework
  - [ ] Actor Module.
  - [ ] RPC framework.
  - [ ] Distributed runtime for actor.

## Design

Marrow 使用 Webassembly 作为基础运行时。并基于 cabi 设计各个模块的接口模式。

## API Manager

API Manager 用于拦截内部环境的调用，并根据权限规则配置决定是否允许进行正常的调用。

## API Level
### Core

Marrow的核心API支持唯一一类API，即内存分配相关的API。对Core模块的调用，一般不需要经过

