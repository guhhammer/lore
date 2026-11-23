# yadm git-crypt

Git Crypt 能够实现 Git 仓库中文件的透明加密和解密。

## Usage
- `yadm git-crypt init` : 初始化仓库以使用 Git Crypt
- `yadm git-crypt add-gpg-user 用户_id` : 使用 GPG 共享仓库
- `yadm git-crypt unlock` : 克隆包含加密文件的仓库后，解锁它们
- `yadm git-crypt export-key 路径/到/密钥文件` : 导出对称密钥

## Links
- More information: https://github.com/AGWA/git-crypt
