# CRUSH.md

## Development Commands

### Frontend
```bash
# Type checking
pnpm type-check

# Linting (oxlint + eslint)
pnpm lint

# Code formatting
pnpm format

# Development server
pnpm dev

# Build frontend
pnpm build
```

### Tauri (Full Stack)
```bash
# Full development environment
pnpm tauri dev

# Production build
pnpm tauri build
```

### Rust Backend
```bash
# From src-tauri directory
cargo build
cargo test
cargo clippy
```

## Code Style Guidelines

### Vue/TypeScript
- Use `@/*` alias for src directory imports
- No semicolons, single quotes, 100 char line width
- Composition API with `<script setup lang="ts">`
- Auto-import Vue/Naive UI APIs via unplugin
- Pinia stores in `stores/` directory
- Define TypeScript interfaces in `types/`

### Rust
- Serde derive for all serializable structs
- Use `AppResult<T>` alias for Result types
- Async functions with `tokio` runtime
- Error handling with `anyhow`/`thiserror`
-snake_case for function/variable names
-PascalCase for struct/enum names

### Architecture
- API layer: `src-tauri/src/api/`
- Services: `src-tauri/src/services/`
- Components: `src/components/{common,layouts,update}/`
- Use Tauri commands for frontend-backend communication
- Naive UI for all UI components