# clear-history

删除 PowerShell 会话命令历史记录中的条目。

## Usage
- `Clear-History` : 清除当前会话中的所有命令历史记录
- `Clear-History -CommandLine "命令"` : 按指定名称清除命令
- `Clear-History -CommandLine "命令1", "命令2", ...` : 按名称清除多个命令
- `Clear-History -Id ID_编号` : 按 ID 清除指定历史记录条目
- `Clear-History -Id id1, id2, ...` : 清除多个 ID
- `Clear-History -Id (起始_ID..结束_ID)` : 清除 ID 范围内的命令
- `Clear-History -WhatIf` : 显示将被删除的内容
- `Clear-History -Confirm` : 清除前请求确认
