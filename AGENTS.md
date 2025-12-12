# Repository Guidelines

## 项目结构与模块
- `src/`：Vue 3 + TypeScript 前端，`components/` 放 UI 片段，`views/` 为页面，`stores/` 存放 Pinia 状态，`services/` 封装前端数据/更新逻辑，`assets/` 负责样式与静态资源。
- `src-tauri/`：Rust 侧命令与服务，`api/` 暴露给前端的命令，`services/` 处理下载、日志、抓取、更新，`utils/` 为路径与压缩工具。`tauri.conf.json` 管理打包与权限。
- `public/` 存放静态公开资产；`docs/` 维护额外说明；`.github/` 为 CI、模板。

## 构建、测试与开发
- 安装依赖：`pnpm install`
- 前端预览：`pnpm dev`（仅 Web 视图），桌面联调用 `pnpm tauri:dev`。
- 类型与构建：`pnpm type-check` 仅类型检查；`pnpm build` 运行类型检查并构建前端；`pnpm tauri:build` 产出可分发安装包。
- 质量检查：`pnpm lint`（oxlint + eslint），`pnpm format` 仅格式化 `src/`。

## 代码风格与命名
- 统一使用 TypeScript/ESM，缩进 2 空格，遵循 `.editorconfig` 与 Prettier/ESLint 规则。
- 组件/布局使用 PascalCase 文件与导出名（如 `MainLayout.vue`），组合式函数用 `useXxx`，状态仓库用 `useXxxStore`。
- Rust 侧遵循 rustfmt 与 clippy 默认习惯；模块文件使用蛇形命名，公共 API 保持简洁注释。
- 路由、枚举与键名优先英文小写-连字符，资源路径使用相对路径明确指向。

## 测试与验证
- 当前未引入自动化测试框架；提交前至少跑通 `pnpm type-check`、`pnpm lint`、`pnpm tauri:dev` 手测核心流程（加载列表、下载、更新检查）。
- 如新增逻辑，优先补充单元/集成测试文件至 `tests/`（若新建目录），或在 PR 描述中附手测场景与结果。

## 提交与 Pull Request
- 提交信息建议语义化英文前缀：`feat: ...`、`fix: ...`、`chore: ...`、`docs: ...`，保持动词祈使句。
- PR 需包含：变更摘要、动机/关联 Issue、主要截图或录屏（涉及 UI 时）、验证步骤（包含执行过的命令）。
- 确保分支已 rebase 到主分支最新，CI/Lint 通过后再请求评审。

## 配置与安全
- 不要提交包含密钥、token、真实下载源的文件；本地敏感配置放 `.env.local` 并写入 `.gitignore`。
- 自动更新逻辑依赖 GitHub Releases，如更换发布源需同步更新 `tauri.conf.json` 与前端 `services/updaterService.ts`。
- 打包前确认 `src-tauri/capabilities/` 与权限配置满足最小授权，避免无用的系统权限暴露。
