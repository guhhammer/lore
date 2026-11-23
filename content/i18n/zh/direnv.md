# direnv

Shell 扩展为加载和卸载环境变量，具体取决于当前目录。

## Usage
- `direnv allow .` : 授予 direnv 当前目录中加载 `.envrc`
- `direnv deny .` : 撤销在当前目录加载 `.envrc` 的授权
- `direnv edit .` : 使用默认编辑器编辑 `.envrc` 并在退出时重载环境
- `direnv reload` : 触发环境重载
- `direnv status` : 打印一些调试状态信息

## Links
- More information: https://github.com/direnv/direnv/blob/master/man/direnv.1.md
