# 开发环境配置

本文档将指导您如何在 Windows、macOS 和 Linux 上配置开发环境，以便运行和构建彩旗剪辑。

## Windows

1. 安装 Rust 和 Cargo：
   - 下载并运行 [rustup-init.exe](https://win.rustup.rs/x86_64)
   - 按照安装向导完成安装

2. 安装 Microsoft Visual C++ 构建工具：
   - 访问 [Microsoft C++ 构建工具](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - 安装"C++ 构建工具"工作负载

3. 安装 WebView2：
   - 下载 [WebView2 运行时](https://developer.microsoft.com/zh-cn/microsoft-edge/webview2/)

4. 安装 Node.js：
   - 下载并安装 [Node.js](https://nodejs.org/)（v18 或更高版本）

5. 安装 Tauri CLI：

   ```cmd
   cargo install tauri-cli --version "^2.0.0" --locked
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

3. 安装 Node.js：

   ```bash
   # 使用 Homebrew
   brew install node@18
   ```

4. 安装 Tauri CLI：

   ```bash
   cargo install tauri-cli --version "^2.0.0" --locked
   ```

## Linux

1. 安装 Rust 和 Cargo：

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. 安装依赖项 (基于 Debian/Ubuntu)：

   ```bash
   sudo apt update
   sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
   ```

   对于其他发行版，请参考 [Tauri 官方文档](https://tauri.app/zh-cn/start/prerequisites)

3. 安装 Node.js：

   ```bash
   # 使用 NodeSource
   curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
   sudo apt install -y nodejs
   ```

4. 安装 Tauri CLI：

   ```bash
   cargo install tauri-cli --version "^2.0.0" --locked
   ```

## 验证安装和克隆项目

完成上述环境配置后，请按照以下步骤验证安装并克隆项目：

1. 克隆项目并进入项目目录：

   ```bash
   git clone https://github.com/sihuangtech/sk-clip-desktop.git
   cd sk-clip-desktop
   ```

2. 验证环境是否成功安装：

   ```bash
   rustc --version
   cargo --version
   node --version
   npm --version
   cargo tauri --version
   ```

3. 根据操作系统执行额外的验证：
   - Windows：确保安装了 WebView2 运行时
   - macOS：运行 `pkgutil --pkg-info=com.apple.pkg.CLTools_Executables` 检查 Xcode 命令行工具
   - Linux：运行 `dpkg -s libwebkit2gtk-4.0-dev libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev | grep 'Status: install ok installed'` 检查系统依赖

如果所有命令都返回适当的版本信息，说明环境已正确配置。
