# tldr

顯示來自 tldr-pages 專案的命令列工具的簡單幫助頁面。

## Usage
- `tldr 命令` : 列印指定命令的 tldr 頁面（提示：這就是你來到這裡的方式！）
- `tldr 命令 子命令` : 列印指定子命令的 tldr 頁面
- `tldr -L 語言代碼 命令` : 用指定語言列印命令的 tldr 頁面（如果沒有，回傳英語）
- `tldr -p android|common|freebsd|linux|osx|netbsd|openbsd|sunos|windows 命令` : 列印指定平台的命令的 tldr 頁面
- `tldr -u` : 更新 tldr 頁面的本地快取
- `tldr -l` : 列出目前平台和 `common` 的所有頁面
- `tldr -l | fzf --preview "tldr {1} --color=always" --preview-window=right,70% | xargs tldr` : 在終端視窗中瀏覽 tldr 頁面（需要有 `fzf`）
- `tldr -l | shuf -n 1 | xargs tldr` : 列印隨機命令的 tldr 頁面
