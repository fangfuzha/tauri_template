# Tauri + Vue + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Commands

### Start (desktop app)

```bash
bun install
bun run tauri dev
```

### Start (frontend only)

```bash
bun run dev
```

### Build (frontend)

```bash
bun run build
```

### Package (desktop app)

```bash
bun run tauri build
```

### Generate bindings

```bash
cargo run --manifest-path src-tauri/Cargo.toml --bin export_bindings
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
