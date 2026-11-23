# choco pin

使用 Chocolatey 将一个包固定到指定的版本。 被固定版本的包会在更新时自动忽略。

## Usage
- `choco pin list` : 显示被固定的包以及他们对应的版本号
- `choco pin add -n 包名` : 将一个包固定至当前版本
- `choco pin add -n 包名 --version 版本号` : 将一个包固定直指定的版本
- `choco pin remove -n 包名` : 移除指定包的固定状态

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/pin/
