# ssh-add

在 ssh 代理中管理加载的 ssh 密钥。 需要确保 ssh 代理已启动并正在运行以加载其中的密钥。

## Usage
- `ssh-add` : 将 `~/.ssh` 中的默认 SSH 密钥添加到 SSH 代理
- `ssh-add 路径/到/私钥文件` : 向 ssh 代理添加指定密钥
- `ssh-add -l` : 列出当前加载的密钥的指纹
- `ssh-add -d 路径/到/私钥文件` : 从 ssh 代理中删除密钥
- `ssh-add -D` : 从 ssh 代理中删除所有当前已有的密钥
- `ssh-add -K 路径/到/私钥文件` : 向 ssh 代理和密钥链添加密钥

## Links
- More information: https://man.openbsd.org/ssh-add
