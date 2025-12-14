# 🖥️ System Info API - Rust 实现的系统信息监控 API

[![Rust](https://img.shields.io/badge/rust-1.65+-orange.svg)](https://www.rust-lang.org/)
[![Actix-web](https://img.shields.io/badge/actix--web-4.0-blue.svg)](https://actix.rs/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

一个使用 Rust 和 Actix-web 框架实现的高性能系统信息监控 API，可以通过 HTTP 接口获取电脑硬件和软件信息。

## ✨ 特性

- ✅ **系统信息** - 操作系统、主机名、运行时间、内核版本
- ✅ **CPU 信息** - 型号、频率、使用率、核心数、负载情况
- ✅ **内存信息** - 总量、使用量、交换空间、使用百分比
- ✅ **磁盘信息** - 分区、文件系统、空间使用情况、挂载点
- ✅ **网络信息** - 接口、MAC地址、流量统计、数据包计数
- ✅ **进程管理** - 进程列表、搜索、资源使用监控
- ✅ **统一响应** - 标准化 JSON 响应格式
- ✅ **跨平台** - 支持 Windows、Linux、macOS
- ✅ **实时监控** - 动态刷新系统状态

## 🚀 快速开始

### 前提条件
- [Rust 1.65+](https://www.rust-lang.org/tools/install)
- Cargo（Rust 包管理器）

### 安装和运行

1. **克隆项目**
```bash
git clone https://github.com/yourusername/systeminfoapi.git
cd systeminfoapi