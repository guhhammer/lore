# apt

以 Debian 為基底的發行版的套件管理器。 旨在作為用於互動式使用的 `apt-get` 使用者友善的替代方案。

## Usage
- `sudo apt update` : 更新套件可用版本列表（推薦在執行其他 `apt` 指令前先執行）
- `apt search package` : 搜尋套件的名稱或描述
- `apt list package` : 只搜尋套件的名稱（支援萬用字元 `*`）
- `apt show package` : 顯示詳細的套件資訊
- `sudo apt install package` : 安裝或是更新套件到最新版本
- `sudo apt remove package` : 移除套件（使用 `purge` 會一併移除設定檔）
- `sudo apt upgrade` : 升級所有安裝的套件到最新的版本
- `apt list -i` : 列出所有安裝的套件

## Links
- More information: https://wiki.archlinux.org/title/Pacman/Rosetta
