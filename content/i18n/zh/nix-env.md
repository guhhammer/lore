# nix-env

管理或查询 Nix 用户环境。

## Usage
- `nix-env -q` : 列出所有已安装的软件包
- `nix-env -q 搜索模式` : 查询已安装的软件包（支持 `正则表达式`）
- `nix-env -qa 搜索模式` : 从 Nixpkgs 注册表中查询可用的软件包
- `nix-env -iA nixpkgs.软件包名称` : 从 Nixpkgs 注册表中安装软件包
- `nix-env -i 软件包名称 -f example.com` : 从自定义 URL 安装软件包
- `nix-env -e 软件包名称` : 卸载软件包
- `nix-env -u 软件包名称` : 升级软件包
- `nix-env --help --选项名称` : 获取特定操作（`--install`、`--upgrade` 等）的使用帮助

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-env.html
