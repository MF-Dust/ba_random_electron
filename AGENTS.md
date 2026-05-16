# Repository Guidelines

## Project Structure & Module Organization

This project is a Tauri 2 desktop app with a Vue 3/Vite renderer.

- `src/` contains the frontend app, including `App.vue`, `main.js`, shared CSS, router setup, Tauri API wrappers, views, and components.
- `src-tauri/` contains the Rust backend, Tauri window/tray setup, commands, capabilities, and package configuration.
- `public/image/` and `public/sound/` contain bundled visual and audio assets.
- `config.yml` is the runtime configuration example used by the app.
- `dist/`, `node_modules/`, and `src-tauri/target/` are generated outputs and should not be edited directly.

## Build, Test, and Development Commands

- `npm ci` installs exact Node dependencies from `package-lock.json`.
- `npm run dev` starts the full Tauri development app.
- `npm run dev:frontend` starts only the Vite renderer on port `5173`.
- `npm run build:frontend` builds the renderer only.
- `npm run build` builds the Windows Tauri package.
- `npm run preview` previews the built Vite frontend.
- `cargo fmt --manifest-path src-tauri/Cargo.toml` formats Rust code.
- `cargo check --manifest-path src-tauri/Cargo.toml` checks Rust compilation without packaging.

## Coding Style & Naming Conventions

Use the existing style: 2-space indentation in Vue, JavaScript, CSS, JSON, and YAML; standard `rustfmt` formatting for Rust. Vue components use PascalCase file names such as `FloatingButton.vue`; route views live in `src/views/`. JavaScript modules use camelCase exports and the `@` alias for `src`. Rust structs and enums use PascalCase, functions and variables use snake_case, and serialized config fields use camelCase via Serde.

## Testing Guidelines

No automated test script is currently defined. Before opening a PR, run `npm run build:frontend` and `cargo check --manifest-path src-tauri/Cargo.toml`; for packaging or release changes, run `npm run build`. If tests are added, place frontend tests near the related renderer module and Rust unit tests inside the relevant `src-tauri/src/*.rs` module.

## Commit & Pull Request Guidelines

Git history uses concise Chinese prefixes such as `功能:`, `修复:`, `优化:`, `项目:`, `版本号:`, and `Agent:`. Keep commits focused and describe user-visible behavior first. Include `[skip build]` only when intentionally bypassing the Windows build workflow; use `[unstable]` for preview release builds.

Pull requests should include a short summary, linked issue when applicable, test/build commands run, and screenshots or recordings for UI changes.

## Security & Configuration Tips

Do not commit personal `config.yml` data, secrets, generated installers, or local build artifacts. Treat files in `public/` as third-party assets and preserve licensing notes in `README.md`.
