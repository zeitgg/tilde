# Tilde

A small Rust desktop app built with [Zed's GPUI](https://gpui.rs/). It opens a native window with a counter and an Increment button.

## Run

Install stable Rust with [rustup](https://rust-lang.org/tools/install/) if needed, then run:

```sh
cargo run
```

On macOS, install the Xcode command line tools with `xcode-select --install` if needed. The `runtime_shaders` feature compiles Metal shaders when the app starts, so a full Xcode installation is not required for this starter.

This machine has a newer SDK than its linker supports. An ignored local `.cargo/config.toml` selects the compatible command line tools SDK. If you encounter an `unknown architecture arm64e.x1-macos` linker error elsewhere, set `SDKROOT` to a compatible installed SDK before running Cargo.

Click **Increment** to update the counter. Close the window or press **Cmd+Q** on macOS (**Ctrl+Q** on Linux) to quit.

## Develop

The application and its root view live in `src/main.rs`. Change `Tilde` to add state and its `Render` implementation to change the UI. Call `cx.notify()` after changing state to request a redraw.

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
```

Zed tasks for running, checking, and formatting the project are included in `.zed/tasks.json`.

The project uses the published GPUI 0.2.2 crate. See its [API docs](https://docs.rs/gpui/0.2.2/gpui/) and the [official examples](https://github.com/zed-industries/zed/tree/main/crates/gpui/examples) for more patterns. Examples on Zed's main branch may use a newer API.
