# win-island Project Instructions & Guidelines

## Architecture Overview
- **App Type:** Windows Desktop Overlay (macOS-style Dynamic Island)
- **Framework:** Tauri v2 (Rust backend) + React 19 / TypeScript (Vite frontend)
- **Animations:** Framer Motion with spring physics (`damping`, `stiffness`, `mass`)

## Git & Release Standards
- All commits must strictly follow **Conventional Commits** (`feat:`, `fix:`, `docs:`, `chore:`).
- Releases are triggered by git tags matching `v*.*.*`.
- Always keep [CHANGELOG.md](./CHANGELOG.md) updated before creating release tags.

## Window Overlay Rules (Tauri v2)
- Main window must be `transparent: true`, `decorations: false`, and `alwaysOnTop: true`.
- Native click-through (`WS_EX_TRANSPARENT`) should be enabled when the island is collapsed/idle and disabled on hover/focus.
