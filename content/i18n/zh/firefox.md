# firefox

一个自由、开源的网络浏览器。

## Usage
- `firefox https://www.duckduckgo.com` : 启动 Firefox 并打开网页
- `firefox --new-window https://www.duckduckgo.com` : 打开新窗口
- `firefox --private-window` : 打开隐私（隐身）窗口
- `firefox --search "wikipedia"` : 使用默认搜索引擎搜索“wikipedia”
- `firefox --safe-mode` : 在安全模式中启动 Firefox, 所有扩展会被禁用
- `firefox --headless --screenshot 路径/到/输出文件.png https://example.com/` : 在无头模式中截取网页截屏
- `firefox --profile 路径/到/文件夹 https://example.com/` : 使用特定的配置文件允许多个单独的 Firefox 实例同时运行
- `firefox --setDefaultBrowser` : 设置 Firefox 为默认浏览器

## Links
- More information: https://wiki.mozilla.org/Firefox/CommandLineOptions
