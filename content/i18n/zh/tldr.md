# tldr

显示来自 tldr-pages 项目的命令行工具的简单帮助页面。

## Usage
- `tldr 命令` : 打印指定命令的 tldr 页面（提示：这就是你来到这里的方式！）
- `tldr 命令 子命令` : 打印指定子命令的 tldr 页面
- `tldr -L 语言代码 命令` : 用指定语言打印命令的 tldr 页面（如果没有，返回英语）
- `tldr -p android|common|freebsd|linux|osx|netbsd|openbsd|sunos|windows 命令` : 打印指定平台的命令的 tldr 页面
- `tldr -u` : 更新 tldr 页面的本地缓存
- `tldr -l` : 列出当前平台和 `common` 的所有页面
- `tldr -l | fzf --preview "tldr {1} --color=always" --preview-window=right,70% | xargs tldr` : 在终端窗口中浏览 tldr 页面（需要有 `fzf`）
- `tldr -l | shuf -n 1 | xargs tldr` : 打印随机命令的 tldr 页面
