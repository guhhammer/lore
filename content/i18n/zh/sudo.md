# sudo

以超级用户或其他用户身份执行单个命令。

## Usage
- `sudo less /var/log/syslog` : 以超级用户身份运行命令
- `sudo -e /etc/fstab` : 以超级用户身份使用默认编辑器编辑文件
- `sudo -u 用户 -g 组 id -a` : 以其他用户和(或)组的身份运行命令
- `sudo !!` : 重复执行上一条以`sudo`为前缀的命令（仅适用于 Bash、Zsh 等）
- `sudo -i` : 以超级用户权限启动默认 Shell，并运行登录特定的文件（如`.profile`、`.bash_profile`等）
- `sudo -s` : 在不改变环境的情况下，使用超级用户权限启动默认的 shell
- `sudo -i -u 用户` : 以指定用户的身份启动默认的 shell，加载该用户的环境变量并读取登录特定的文件（如`.profile`、`.bash_profile`等）
- `sudo -ll` : 列出调用用户允许（和禁止）的命令

## Links
- More information: https://www.sudo.ws/sudo.html
