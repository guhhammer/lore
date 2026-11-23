# nix classic

Nix 包管理器经典、稳定的接口，使得包管理变得可靠、可复现且可声明。 部分 Nix 命令，例如 `nix-build`、`nix-shell`、`nix-env` 和 `nix-store`，拥有各自的页面。

## Usage
- `nix-env -qaP 搜索词正则表达式` : 通过包名在 nixpkgs 中搜索包
- `nix-shell -p 包1 包2 包3 ...` : 启动一个包含指定可用的 shell
- `nix-env -iA nixpkgs.包1 nixpkgs.包2 ...` : 永久安装某些包
- `nix-store -q --tree /nix/store/校验和-包名-版本.扩展名` : 以树状格式展示存储路径（包）的所有依赖
- `nix-channel --update` : 更新频道（仓库源）
- `nix-collect-garbage` : 从 Nix 存储中移除未使用的路径

## Links
- More information: https://nixos.org/
