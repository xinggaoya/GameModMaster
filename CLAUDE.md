# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

GameModMaster 是一个基于 Tauri + Vue 3 + TypeScript 开发的游戏修改器管理工具，主要用于管理风灵月影游戏修改器。项目采用前后端分离架构，前端使用 Vue 3 + Naive UI，后端使用 Rust Tauri。

## 开发命令

### 基础开发
```bash
# 安装依赖
pnpm install

# 启动开发环境
pnpm tauri dev

# 构建生产版本
pnpm tauri build

# 仅前端开发
pnpm dev

# 仅前端构建
pnpm build
```

### 代码质量
```bash
# 类型检查
pnpm type-check

# 代码检查和修复
pnpm lint

# 格式化代码
pnpm format

# 预览构建结果
pnpm preview
```

## 架构设计

### 前端架构 (Vue 3)
- **路由系统**: 使用 Vue Router，包含三个主要页面：
  - `/` - 首页 (HomeView): 显示游戏修改器列表
  - `/detail/:id` - 详情页 (DetailView): 显示修改器详细信息
  - `/downloads` - 下载页 (DownloadsView): 管理下载任务

- **状态管理**: 使用 Pinia 进行状态管理
  - `trainer.ts`: 管理修改器相关状态，包含缓存机制和本地存储

- **组件结构**:
  - `components/layouts/`: 布局组件 (MainLayout, WindowLayout)
  - `components/common/`: 通用组件 (GameCard, TrainerCard, ActionButton等)
  - `components/update/`: 更新相关组件 (UpdateDialog, CheckUpdateButton)

- **服务层**:
  - `services/updaterService.ts`: 应用更新服务
  - `utils/errorHandler.ts`: 统一错误处理

### 后端架构 (Rust Tauri)
- **API层**: 处理前端调用
  - `api/trainer.rs`: 修改器相关API
  - `api/updater.rs`: 更新相关API
  - `api/admin.rs`: 管理员权限API
  - `api/error.rs`: 错误定义和处理

- **服务层**: 业务逻辑
  - `services/trainer.rs`: 修改器管理服务
  - `services/download_manager.rs`: 下载管理服务
  - `services/scraper.rs`: 网页抓取服务
  - `services/updater.rs`: 更新服务
  - `services/logger.rs`: 日志服务

- **工具层**: 辅助功能
  - `utils/path.rs`: 路径处理
  - `utils/zip.rs`: 压缩文件处理

## 核心功能

### 修改器管理
- 从远程获取修改器列表
- 搜索和筛选修改器
- 下载和安装修改器
- 启动已安装的修改器
- 管理已安装的修改器

### 数据缓存
- 前端实现15分钟数据缓存机制
- 使用 localStorage 进行本地数据持久化
- 支持离线查看已下载的修改器信息

### 错误处理
- 统一的错误代码定义 (1000-11000)
- 根据错误严重程度显示不同类型的提示
- 完整的错误日志记录

### 自动更新
- 检查 GitHub Releases 获取最新版本
- 支持增量下载和自动安装
- 实时显示下载进度

## 技术特性

### 前端技术栈
- Vue 3 + TypeScript
- Naive UI 组件库
- Pinia 状态管理
- Vue Router 路由
- Vite 构建工具
- 自动导入组件和API

### 后端技术栈
- Rust + Tauri 2.x
- serde JSON 序列化
- tokio 异步运行时
- reqwest HTTP 客户端
- zip 文件处理

### 开发配置
- 使用 `@` 别名指向 `src` 目录
- Naive UI 自动导入和解析
- ESLint + Oxlint 代码检查
- Prettier 代码格式化
- TypeScript 严格模式

## 重要文件说明

- `src/stores/trainer.ts`: 核心状态管理，包含缓存逻辑
- `src/services/updaterService.ts`: 更新功能的前端实现
- `src/utils/errorHandler.ts`: 错误处理的统一实现
- `src-tauri/src/services/download_manager.rs`: 下载管理核心逻辑
- `src-tauri/src/services/scraper.rs`: 网页数据抓取
- `vite.config.ts`: Vite 配置，包含 Tauri 专用设置
- `tauri.conf.json`: Tauri 应用配置

## 调试和测试

### 开发模式
- 使用 `pnpm tauri dev` 启动完整开发环境
- 前端热重载端口固定为 5173
- Tauri 应用会自动连接到开发服务器

### 错误调试
- 前端错误通过 Naive UI message 显示
- 后端错误记录到控制台和日志文件
- 使用浏览器开发者工具调试前端代码

### 性能优化
- 图片懒加载和缓存
- 组件级别的代码分割
- Tauri 生产构建优化
- 资源压缩和混淆