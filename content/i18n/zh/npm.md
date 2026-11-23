# npm

JavaScript 和 Node.js 软件包管理器。 管理 Node.js 项目及其模块依赖项。

## Usage
- `npm init -y` : 使用默认值创建一个 `package.json` 文件（省略 `--yes` 来以交互方式执行）
- `npm i` : 下载 `package.json` 中列出的所有依赖项软件包
- `npm i 软件包名@版本号` : 下载软件包的特定版本并将其添加到 `package.json` 中的依赖项列表中
- `npm i 软件包名 -D` : 下载软件包的最新版本并将其添加到 `package.json` 中的开发依赖项列表中
- `npm i 软件包名 -g` : 下载软件包的最新版本并全局安装（用 `npm config set prefix` 设置安装位置）
- `npm r 软件包名` : 卸载软件包并将其从 `package.json` 中的依赖项列表中删除
- `npm ls` : 列出所有本地安装的依赖项
- `npm ls -g --depth 0` : 列出所有顶层全局安装的软件包

## Links
- More information: https://docs.npmjs.com/cli/npm/
