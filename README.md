# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Automatic Tailwind (Dioxus 0.7+)

As of Dioxus 0.7, there no longer is a need to manually install tailwind. Simply `dx serve` and you're good to go!

Automatic tailwind is supported by checking for a file called `tailwind.css` in your app's manifest directory (next to Cargo.toml). To customize the file, use the dioxus.toml:

```toml
[application]
tailwind_input = "my.css"
tailwind_output = "assets/out.css" # also customize the location of the out file!
```

### Tailwind Manual Install

To use tailwind plugins or manually customize tailwind, you can can install the Tailwind CLI and use it directly.

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation/tailwind-cli
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

You run the bundle command from your terminal. 
Note: To create a Windows installer, you must be on Windows; 
for macOS, you must be on a Mac.
To export it to an executable file, use `--release` flag. E.g.
```bash
dx bundle --platform desktop --release
```

You will find a `.msi` installer 
in `target/dx/dioxus-autotrade/release/bundle/msi/`.
ou will find a `.app` folder and potentially a `.dmg` 
in `target/dx/dioxus-autotrade/release/bundle/macos/`.

### Bundle
1. 准备工作（安装 Target）
首先，确保你的开发机安装了 Intel 架构的编译工具链：
```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

2. 执行打包命令
在你的 Dioxus 项目根目录下运行：
```bash
dx bundle --platform desktop --release --target x86_64-apple-darwin
```

💡 进阶建议：设置最低系统版本
有时候即使架构对了，如果你的 SDK 太新（比如用的 macOS 15 的 SDK），客户如果是几年前的老系统（如 macOS 10.15）依然打不开。
为了提高兼容性，建议在执行命令前，通过环境变量指定最低支持的 macOS 版本（通常建议 11.0 或更低）：
```bash
dx bundle --platform desktop --release --target aarch64-apple-darwin
dx bundle --platform desktop --release --target aarch64-apple-darwin --package-types macos --package-types dmg
```