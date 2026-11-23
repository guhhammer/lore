# brew bundle

Homebrew，Homebrew Cask 和 Mac App Store 的打包器。

## Usage
- `brew bundle` : 从当前路径下的 Brewfile 安装软件包
- `brew bundle --file 路径/到/文件` : 从特定路径下的特定 Brewfile 安装软件包
- `brew bundle dump` : 基于所有安装的软件包，创建 Brewfile
- `brew bundle cleanup --force` : 卸载所有 Brewfile 中未列出的软件包
- `brew bundle check` : 检查 Brewfile 中指定的包，是否需要安装或升级
- `brew bundle list --all` : 列出 Brewfile 中的所有安装包条目

## Links
- More information: https://docs.brew.sh/Manpage#bundle-subcommand
