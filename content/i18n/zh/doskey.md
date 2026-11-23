# doskey

管理宏，Windows 命令和命令行。

## Usage
- `doskey /macros` : 列出可用的宏
- `doskey 宏的名称 = "命令"` : 创建一个新的宏
- `doskey /exename=可执行文件名 宏的名称 = "命令"` : 为指定可执行文件创建新的宏
- `doskey 宏的名称 =` : 删除一个宏
- `doskey /history` : 列出所有储存在内存中的命令
- `doskey /macros >保存宏的文件名` : 将宏保存到文件以便于移植
- `doskey /macrofile = 保存宏的文件名` : 从文件中加载宏
