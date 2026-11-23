# choco upgrade

使用 Chocolatey 升级一个或多个包。

## Usage
- `choco upgrade 包名 包名 ..` : 升级一个或多个用空格分隔的软件包
- `choco upgrade 包名 --version 版本号` : 将一个包升级到指定版本
- `choco upgrade all` : 升级全部包
- `choco upgrade all --except "包名 , 包名 .."` : 升级除指定的用逗号分隔的包之外的所有包
- `choco upgrade 包名 -y` : 自动确认所有提示
- `choco upgrade 包名 -s 源url|别名` : 从自定义源处升级包
- `choco upgrade 包 -u 用户名 -p 密码` : 提供一个用户名和密码来进行验证

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/upgrade/
