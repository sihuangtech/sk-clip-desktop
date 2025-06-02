# 更新 Rust 和 Cargo

本文档将指导您如何更新 Rust 和 Cargo 到最新版本。

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