# git config

管理 Git 仓库的自定义设置项。 这些设置可以分为局部设置（只对当前仓库生效）和全局设置（对当前用户生效）。

## Usage
- `git config -l --local` : 列出局部设置项（存储在当前仓库的 `.git/config`）
- `git config -l --global` : 列出全局配置项（存储在 `~/.gitconfig`）
- `git config -l --system --show-origin` : 列出系统配置项（存储在 `/etc/gitconfig`），并且展示文件的位置
- `git config alias.unstage` : 获取某个配置项的值
- `git config --global alias.unstage "reset HEAD --"` : 设置某个全局配置项
- `git config --global --unset alias.unstage` : 将某个全局配置项恢复为默认值
- `git config -e` : 使用默认编辑器修改本地设置
- `git config --global -e` : 使用默认编辑器修改全局设置

## Links
- More information: https://git-scm.com/docs/git-config
