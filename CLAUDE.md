# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

GameModMaster 是一个基于 Tauri + Vue 3 + TypeScript 开发的游戏修改器管理工具，主要用于管理风灵月影游戏修改器。项目采用前后端分离架构，前端使用 Vue 3 + Naive UI，后端使用 Rust Tauri。

## 开发命令

### 基础开发
```bash
# 安装依赖
pnpm install

# 启动开发环境 (前后端一起)
pnpm tauri dev

# 构建生产版本
pnpm tauri build

# 仅前端开发
pnpm dev

# 仅前端构建
pnpm build

# 启动开发环境 (别名)
pnpm tauri:dev

# 构建生产版本 (别名)
pnpm tauri:build
```

### 代码质量
```bash
# 类型检查
pnpm type-check

# 运行所有代码检查 (oxlint + eslint)
pnpm lint

# 运行 oxlint 检查
pnpm lint:oxlint

# 运行 eslint 检查
pnpm lint:eslint

# 格式化代码
pnpm format

# 预览构建结果
pnpm preview
```

## 架构设计

### 前端架构 (Vue 3)
- **路由系统**: 使用 Vue Router，包含四个主要页面：
  - `/` - 首页 (HomeView): 显示游戏修改器列表
  - `/detail/:id` - 详情页 (DetailView): 显示修改器详细信息
  - `/downloads` - 收藏页 (DownloadsView): 管理已下载的修改器
  - `/settings` - 设置页 (SettingsView): 应用配置管理

- **状态管理**: 使用 Pinia 进行状态管理
  - `trainer.ts`: 管理修改器相关状态，包含缓存机制和本地存储
  - `counter.ts`: 计数器状态（示例）

- **国际化支持**: 使用 vue-i18n 支持多语言
  - 支持语言：简体中文、英语、西班牙语、法语、日语
  - 自动检测浏览器语言，支持手动切换

- **组件结构**:
  - `components/layouts/`: 布局组件 (MainLayout, WindowLayout)
  - `components/common/`: 通用组件 (GameCard, TrainerCard, ActionButton, StatusCard, DownloadProgress等)
  - `components/update/`: 更新相关组件 (UpdateDialog, CheckUpdateButton)

- **服务层**:
  - `services/updaterService.ts`: 应用更新服务
  - `services/storageService.ts`: 本地存储服务
  - `utils/errorHandler.ts`: 统一错误处理

- **类型定义**:
  - `types/index.ts`: 主要类型定义
  - `types/tauri.d.ts`: Tauri API 类型定义

### 后端架构 (Rust Tauri)
- **API层**: 处理前端调用
  - `api/trainer.rs`: 修改器相关API
  - `api/updater.rs`: 更新相关API
  - `api/admin.rs`: 管理员权限API
  - `api/settings.rs`: 设置相关API
  - `api/storage.rs`: 存储相关API
  - `api/error.rs`: 错误定义和处理
  - `api/mod.rs`: Mod管理相关API

- **服务层**: 业务逻辑
  - `services/trainer.rs`: 修改器管理服务
  - `services/download_manager.rs`: 下载管理服务（支持并发下载）
  - `services/scraper.rs`: 网页抓取服务
  - `services/updater.rs`: 更新服务
  - `services/logger.rs`: 日志服务
  - `services/storage.rs`: 存储管理服务
  - `services/settings.rs`: 设置管理服务
  - `services/mod.rs`: Mod管理服务

- **数据模型**:
  - `models/trainer.rs`: 修改器数据模型
  - `models/mod.rs`: Mod数据模型

- **工具层**: 辅助功能
  - `utils/path.rs`: 路径处理
  - `utils/zip.rs`: 压缩文件处理

- **系统托盘**: 支持显示/隐藏主窗口、退出应用等操作

## 核心功能

### 修改器管理
- 从远程获取修改器列表（使用网页抓取技术）
- 搜索和筛选修改器（支持缓存搜索结果）
- 下载和安装修改器（支持断点续传）
- 启动已安装的修改器
- 管理已下载的修改器（删除、查看详情等）
- 支持收藏功能，管理常用修改器

### 数据缓存
- 前端实现15分钟数据缓存机制
- 使用 Tauri Store API 进行数据持久化（从 localStorage 迁移）
- 支持离线查看已下载的修改器信息
- 自动清理过期缓存

### 错误处理
- 统一的错误代码定义 (1000-11000)
- 根据错误严重程度显示不同类型的提示
- 完整的错误日志记录（支持导出日志）

### 自动更新
- 检查 GitHub Releases 获取最新版本
- 支持增量下载和自动安装
- 实时显示下载进度
- 支持更新检查按钮和系统托盘通知

### 国际化
- 支持多语言界面（简体中文、英语、西班牙语、法语、日语）
- 自动检测系统语言
- 语言设置持久化存储

### 设置管理
- 下载路径配置
- 主题切换（浅色/深色/跟随系统）
- 界面语言设置
- 下载完成后自动打开文件夹选项

## 技术特性

### 前端技术栈
- Vue 3 + TypeScript（Composition API）
- Naive UI 组件库（现代化设计）
- Pinia 状态管理（轻量级）
- Vue Router 路由（懒加载）
- Vite 构建工具（快速热重载）
- vue-i18n 国际化支持
- unplugin-auto-import 自动导入
- unplugin-vue-components 组件自动导入

### 后端技术栈
- Rust + Tauri 2.x（跨平台桌面应用）
- serde JSON 序列化
- tokio 异步运行时
- reqwest HTTP 客户端（支持流式下载）
- scraper 网页抓取库
- zip 文件处理
- chrono 时间处理
- thiserror 错误处理
- directories 系统目录获取
- Windows API 集成（窗口管理）

### 开发配置
- 使用 `@` 别名指向 `src` 目录
- Naive UI 自动导入和解析
- ESLint + Oxlint 代码检查（严格模式）
- Prettier 代码格式化
- TypeScript 严格模式
- 端口配置：前端开发服务器固定为 5173
- PNPM 包管理器（提升安装速度）

## 重要文件说明

### 前端核心文件
- `src/stores/trainer.ts`: 核心状态管理，包含缓存逻辑
- `src/services/updaterService.ts`: 更新功能的前端实现
- `src/services/storageService.ts`: 存储服务封装
- `src/utils/errorHandler.ts`: 错误处理的统一实现
- `src/i18n/index.ts`: 国际化配置和语言检测
- `src/router/index.ts`: 路由配置（懒加载）
- `vite.config.ts`: Vite 配置，包含 Tauri 专用设置和自动导入

### 后端核心文件
- `src-tauri/src/main.rs`: 主入口，包含系统托盘和 Tauri 初始化
- `src-tauri/src/services/download_manager.rs`: 下载管理核心逻辑（支持并发）
- `src-tauri/src/services/scraper.rs`: 网页数据抓取（使用 scraper 库）
- `src-tauri/src/services/storage.rs`: 数据持久化管理
- `src-tauri/src/api/trainer.rs`: 修改器相关 API 实现
- `src-tauri/src/api/storage.rs`: 存储 API 对外接口
- `tauri.conf.json`: Tauri 应用配置（窗口、权限等）

## 调试和测试

### 开发模式
- 使用 `pnpm tauri dev` 启动完整开发环境（前后端一起）
- 前端热重载端口固定为 5173
- Tauri 应用会自动连接到开发服务器
- 支持仅前端开发：`pnpm dev`

### 错误调试
- 前端错误通过 Naive UI message 显示（友好提示）
- 后端错误记录到控制台和日志文件（详细日志）
- 使用浏览器开发者工具调试前端代码
- 支持日志查看器组件和日志导出功能

### 性能优化
- 图片懒加载和缓存
- 组件级别的代码分割（路由懒加载）
- Tauri 生产构建优化
- 资源压缩和混淆
- 数据缓存机制（15分钟）
- 并发下载支持
- 组件自动导入减少包体积

### 构建说明
- 生产构建会自动忽略开发依赖
- Windows 平台会生成 `.exe` 安装包
- 使用 `pnpm build` 仅构建前端资源
- 使用 `pnpm tauri build` 构建完整应用

### 端口和权限
- 开发服务器端口：5173（硬编码以避免端口冲突）
- 需要网络权限（下载修改器）
- 需要文件系统权限（读写下载目录）
- Windows 平台需要管理员权限（部分修改器启动）