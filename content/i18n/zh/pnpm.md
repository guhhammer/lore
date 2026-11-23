# pnpm

快速、节省磁盘空间的 Node.js 包管理器。 管理 Node.js 项目和其模块依赖。

## Usage
- `pnpm init` : 创建 `package.json` 文件
- `pnpm i` : 下载 `package.json` 中列出的所有依赖包
- `pnpm add 模块名@版本` : 下载指定版本的包并添加至 `package.json` 的依赖列表
- `pnpm add 模块名 -D` : 下载包并添加至 `package.json` 的开发依赖列表
- `pnpm add 模块名 -g` : 下载并全局安装包
- `pnpm rm 模块名` : 卸载包并将其从 `package.json` 的依赖列表删除
- `pnpm ls` : 打印本地安装的模块树
- `pnpm ls -g --depth 0` : 列出顶级全局安装的模块

## Links
- More information: https://pnpm.io/pnpm-cli
