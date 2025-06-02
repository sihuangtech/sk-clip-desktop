# 调试和构建应用 (Tauri v2)

本文档基于 Tauri v2 版本，将指导您如何运行、调试和构建视频创作与翻译工具的桌面应用。

## 运行应用 (开发模式)

在项目开发过程中，您可以使用开发模式运行应用。这会启动一个本地服务器，并在修改代码时自动刷新，方便进行调试和测试。这个命令在 Tauri v2 中保持不变。

在项目根目录执行以下命令：

```bash
cargo tauri dev
```

这将：
1. 构建后端 Rust 代码。
2. 启动前端 Trunk 开发服务器。
3. 启动 Tauri 应用窗口，加载前端界面。

在开发模式下，您可以在浏览器开发者工具中调试前端界面，并在终端中查看后端 Rust 的输出（例如使用 `println!` 或 `log::info!` 宏打印的信息）。

## 构建应用 (发布版本)

当您准备好发布应用时，可以使用构建命令生成适用于目标操作系统的安装包或可执行文件。Tauri v2 除了支持桌面平台，还增加了对移动平台 (iOS/Android) 的实验性支持。

默认情况下，`cargo tauri build` 命令会构建适用于您当前操作系统的桌面版本。

要为特定的桌面操作系统和架构构建，您可以使用 `--target` 参数。您需要指定一个 Rust target triplete。

在项目根目录执行以下命令：

```bash
cargo tauri build --target <target-triplete>
```

例如，构建桌面平台：

*   **Windows (64位)**：
    ```bash
    cargo tauri build --target x86_64-pc-windows-msvc
    ```

*   **macOS (Intel)**：
    ```bash
    cargo tauri build --target x86_64-apple-darwin
    ```

*   **macOS (Apple Silicon)**：
    ```bash
    cargo tauri build --target aarch64-apple-darwin
    ```

*   **Linux (64位)**：
    ```bash
    cargo tauri build --target x86_64-unknown-linux-gnu
    ```

您可以使用 `rustup target list` 命令查看所有可用的 target triplete，以及哪些已经安装。

下表概览了在不同主机操作系统下可以构建的主要目标平台（需要安装相应的 Rust target toolchain）：

| 主机操作系统 | 可构建目标平台                                                                     | 备注                                                                                                   |
|--------------|------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------|
| Windows      | Windows (x86_64), Linux (x86_64-unknown-linux-gnu), macOS (x86_64-apple-darwin)* | 构建 Linux/macOS 需要安装相应的 Rust target toolchain 和可能的额外工具。                                  |
| macOS        | macOS (x86_64, aarch64), Windows (x86_64-pc-windows-gnu, x86_64-pc-windows-msvc), Linux (x86_64-unknown-linux-gnu) | 构建 Windows 需要安装相应的 Rust target toolchain (gnu 或 msvc)。构建 Linux 可能需要额外的工具和配置。 |
| Linux        | Linux (x86_64-unknown-linux-gnu, 等), Windows (x86_64-pc-windows-gnu, x86_64-pc-windows-msvc)*, macOS (x86_64-apple-darwin)* | 构建 Windows/macOS 需要安装相应的 Rust target toolchain 和可能的额外工具。主要取决于发行版和 glibc 版本。 |

通常，要构建其他平台的应用，需要先使用 `rustup target add <target-triplete>` 安装相应的 Rust target toolchain。

虽然存在跨平台编译的技术，但在 Tauri 应用中（特别是涉及前端构建和系统依赖时）可能会非常复杂，并且需要安装额外的工具链和依赖项。

如果您需要在非目标平台上进行构建，请查阅 Tauri 官方文档中关于 [跨平台编译](https://tauri.app/zh-cn/v1/guides/building/cross-platform) (注意 Tauri v2 可能有更新的指南) 的详细指南，并准备好解决潜在的依赖和配置问题。

构建完成后的输出文件通常位于 `src-tauri/target/<target-triplete>/release/bundle/` 目录下 (移动平台输出路径可能不同)。

## 调试技巧

*   **前端调试**：在开发模式下，您可以按 `F12` (或 `Cmd + Option + I` on macOS) 打开开发者工具，像调试网页一样调试您的 Yew 前端代码。您可以在 Sources 标签页设置断点，查看变量值，使用 Console 标签页打印信息。

*   **后端调试**：
    *   使用 `println!` 或 `log::info!` 在 Rust 代码中打印信息，这些信息会在运行 `cargo tauri dev` 的终端中显示。
    *   配置您的 IDE (如 VS Code + Rust Analyzer 扩展) 进行 Rust 代码的断点调试。您通常需要在 `launch.json` 文件中配置一个针对 Tauri 应用的调试配置。

*   **Tauri API 调用**：当调用后端命令 (invoke) 或监听事件 (emit/listen) 时，可以在前端和后端都添加日志，追踪调用是否成功以及参数和返回值是否正确。

## 构建注意事项

*   **系统依赖**：确保您的系统安装了所有必需的构建依赖项（请参考 [开发环境配置](development-setup.md) 文档）。
*   **代码签名**：对于发布版本，强烈建议进行代码签名，以提高用户信任和兼容性。这需要额外的配置和证书。
*   **优化**：发布版本会进行编译器优化，以提高性能和减小文件大小。调试信息默认会被移除。

如果在构建过程中遇到错误，请检查终端输出的详细错误信息，并参考 [环境安装故障排除](../README.md#环境安装故障排除) 文档。 