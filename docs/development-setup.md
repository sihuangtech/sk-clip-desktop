# 开发环境配置

本文档将指导您如何在 Windows、macOS 和 Linux 上配置开发环境，以便运行和构建视频创作与翻译工具。

## Windows

1. 安装 Rust 和 Cargo：
   - 下载并运行 [rustup-init.exe](https://win.rustup.rs/x86_64)
   - 按照安装向导完成安装

2. 安装 Microsoft Visual C++ 构建工具：
   - 访问 [Microsoft C++ 构建工具](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - 安装"C++ 构建工具"工作负载

3. 安装 WebView2：
   - 下载 [WebView2 运行时](https://developer.microsoft.com/zh-cn/microsoft-edge/webview2/)

4. 安装 Trunk (用于构建Yew应用)：
   ```cmd
   # 使用官方源安装
   cargo install trunk

   # 使用国内镜像源安装
   # 方法1：全局配置（在 ~/.cargo/config.toml 中添加以下内容）：
   # [source.crates-io]
   # replace-with = 'ustc'
   # 
   # [source.ustc]
   # registry = "https://mirrors.ustc.edu.cn/crates.io-index"
   
   # 方法2：项目级配置（在项目根目录创建 .cargo/config.toml）：
   # [source.crates-io]
   # replace-with = 'ustc'
   # 
   # [source.ustc]
   # registry = "https://mirrors.ustc.edu.cn/crates.io-index"
   
   # 然后安装 trunk
   cargo install trunk

   # 其他可用的镜像源配置：
   # 清华大学镜像
   # [source.crates-io]
   # replace-with = 'tuna'
   # 
   # [source.tuna]
   # registry = "https://mirrors.tuna.tsinghua.edu.cn/crates.io-index"
   ```

5. 添加 WebAssembly 目标：
   ```cmd
   rustup target add wasm32-unknown-unknown
   ```

6. 安装 Tauri CLI：
   ```cmd
   cargo install tauri-cli
   ```

## macOS

1. 安装 Rust 和 Cargo：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. 安装开发依赖：
   ```bash
   xcode-select --install
   ```

3. 安装 Trunk (用于构建Yew应用)：
   ```bash
   cargo install trunk
   ```

4. 添加 WebAssembly 目标：
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

5. 安装 Tauri CLI：
   ```bash
   cargo install tauri-cli
   ```

## Linux

1. 安装 Rust 和 Cargo：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. 安装依赖项 (基于Debian/Ubuntu)：
   ```bash
   sudo apt update
   sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
   ```
   
   对于其他发行版，请参考 [Tauri 官方文档](https://tauri.app/zh-cn/start/prerequisites)

3. 安装 Trunk (用于构建Yew应用)：
   ```bash
   cargo install trunk
   ```

4. 添加 WebAssembly 目标：
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

5. 安装 Tauri CLI：
   ```bash
   cargo install tauri-cli
   ```

## 验证安装和克隆项目

完成上述环境配置后，请按照以下步骤验证安装并克隆项目：

1. 克隆项目并进入项目目录：
   ```bash
   git clone <repository-url>
   cd multisay-tauri-desktop-app
   ```

2. 验证环境是否成功安装：
   ```bash
   rustc --version
   cargo --version
   trunk --version
   rustup target list --installed
   ```

3. 根据操作系统执行额外的验证：
   - Windows：确保在 `rustup target list --installed` 输出中看到 `wasm32-unknown-unknown`
   - macOS：运行 `pkgutil --pkg-info=com.apple.pkg.CLTools_Executables` 检查Xcode命令行工具
   - Linux：运行 `dpkg -s libwebkit2gtk-4.0-dev libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev | grep 'Status: install ok installed'` 检查系统依赖

如果所有命令都返回适当的版本信息，并且系统特定的验证也通过，说明环境已正确配置。 