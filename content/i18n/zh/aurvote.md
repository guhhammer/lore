# aurvote

为 AUR 中的包投票。 为了投票成功，文件 `~/.config/aurvote` 必须存在并包含你的 AUR 身份凭证。

## Usage
- `aurvote --configure` : 交互式创建包含你的 AUR 用户名和密码的 `~/.config/aurvote` 文件
- `aurvote package1 package2 ...` : 为一个或多个 AUR 包投票
- `aurvote -u package1 package2 ...` : 为一个或多个 AUR 包取消投票
- `aurvote -c package1 package2 ...` : 检查一个或多个 AUR 包是否已投票
- `aurvote -h` : 显示帮助

## Links
- More information: https://github.com/archlinuxfr/aurvote#name
