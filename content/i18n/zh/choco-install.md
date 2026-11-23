# choco install

使用 Chocolatey 下载一个或多个包。

## Usage
- `choco install 包名 包名 ..` : 安装一个或多个用空格分隔的软件包
- `choco install 配置文件的路径` : 从一个自定义的配置文件中安装包
- `choco install 文件的路径` : 安装一个特定的 `.nuspec` 或 `.nupkg` 文件
- `choco install 包名 --version 版本号` : 安装一个指定版本的包
- `choco install 包名 --allow-multiple` : 允许安装一个包的多个版本
- `choco install 包名 -y` : 自动确认所有提示
- `choco install 包名 -s 源url|别名` : 从自定义的源处获取包
- `choco install 包名 -u 用户名 -p 密码` : 提供一个用户名和密码来进行验证

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/install/
