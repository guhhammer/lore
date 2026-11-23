# eim

安装和管理 ESP-IDF。

## Usage
- `eim install` : 在默认位置（Windows 上为 `C:\esp`，POSIX 系统上为 `~/.espressif`）安装默认（最新）的 ESP-IDF 版本
- `eim install -i v5.3.2` : 非交互式安装特定 ESP-IDF 版本
- `eim wizard` : 运行交互式、指导性的安装向导
- `eim install -i v5.3.2 -p /opt/esp-idf -n false` : 安装特定版本到自定义路径，并强制进入交互模式（以提示选择）
- `eim list` : 列出所有当前已安装的 ESP-IDF 版本
- `eim remove v5.3.2` : 移除一个特定已安装的 ESP-IDF 版本
- `eim install -c path/to/config.toml` : 使用 TOML 配置文件中定义的所有选项以无头模式安装
- `eim install --use-local-archive path/to/archive.zst` : 使用预先下载的归档文件进行离线安装

## Links
- More information: https://docs.espressif.com/projects/idf-im-ui/en/latest/cli_commands.html
