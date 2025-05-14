# 视频翻译工具

这是一个基于 Tauri 和 Rust 开发的视频翻译桌面应用，主要功能是将上传的视频翻译成其他语言，同时尽可能保留原始音色和情感。

## 功能特点

- 支持视频上传和翻译处理
- 支持中文、英文、日语和韩语之间的互译
- 离线处理，保护隐私
- 简洁直观的用户界面

## 技术栈

- 前端：Yew (Rust WebAssembly 框架)
- 后端：Rust + Tauri
- 视频处理：FFmpeg (设计中，当前版本为模拟实现)
- 语音识别：Whisper (设计中，当前版本为模拟实现)
- 机器翻译：开源翻译模型 (设计中，当前版本为模拟实现)
- 语音合成：开源TTS模型 (设计中，当前版本为模拟实现)
- 口型同步：开源口型同步技术 (设计中，当前版本为模拟实现)

## 开发环境配置

### Windows

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

### macOS

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

### Linux

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

## 更新 Rust 和 Cargo

如果您需要更新 Rust 和 Cargo 到最新版本，请按照以下步骤操作：

1. 检查当前版本：
   ```bash
   cargo --version
   rustc --version
   ```

2. 更新到最新稳定版本：
   ```bash
   rustup update
   ```

3. 更新完成后验证版本：
   ```bash
   cargo --version
   rustc --version
   ```

如果您想使用其他版本：
- 更新到最新的 nightly 版本：`rustup update nightly`
- 更新到最新的 beta 版本：`rustup update beta`

注意：
- 更新过程会自动更新所有已安装的工具链
- 如果遇到权限问题，在 Linux/macOS 上可能需要使用 `sudo`
- 在 Windows 上，建议以管理员身份运行命令提示符
- 如果更新后遇到问题，可以使用 `rustup self uninstall` 完全卸载，然后重新安装

## 环境安装故障排除

如果在验证环境时遇到问题，请参考以下排除步骤：

1. **Rust/Cargo 未找到**：
   - 确保已将 Rust 添加到系统 PATH 中
   - Windows 用户可能需要重新启动终端或计算机
   - Linux/macOS 用户确保已执行 `source $HOME/.cargo/env`

2. **Trunk 未找到**：
   - 确保 `$HOME/.cargo/bin` 目录在系统 PATH 中
   - 尝试重新安装：`cargo install trunk --force`

3. **WebAssembly 目标未安装**：
   - 再次运行：`rustup target add wasm32-unknown-unknown`
   - 确保网络连接正常

4. **系统依赖缺失 (Linux)**：
   - 确保已安装所有必要的系统库：
     ```bash
     sudo apt update && sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
     ```

5. **Visual C++ 构建工具问题 (Windows)**：
   - 重新安装 Visual C++ 构建工具
   - 确保已安装 "Desktop development with C++"
   
6. **WebView2 问题 (Windows)**：
   - 确保 WebView2 运行时已正确安装
   - 尝试重启系统

## 运行应用

无论您使用哪种操作系统，都可以通过以下命令运行和构建应用：

1. 开发模式运行：
   ```bash
   cargo tauri dev
   ```

2. 构建发布版本：
   ```bash
   cargo tauri build
   ```

## 使用说明

1. 启动应用后，点击文件选择区域上传视频文件
2. 选择源语言和目标语言
3. 点击"开始翻译"按钮
4. 等待翻译处理完成
5. 完成后可以直接在应用中播放翻译后的视频

## 当前限制

- 当前版本为原型演示，视频处理功能为模拟实现
- 尚未集成真实的语音识别、翻译和语音合成组件
- 未来版本将集成实际的开源视频处理模块

## 许可证

此项目采用 MIT 许可证 - 详情请参阅 LICENSE 文件
