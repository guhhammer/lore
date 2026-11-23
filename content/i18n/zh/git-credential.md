# git credential

用于管理和存储 Git 用户凭证（如用户名和密码）。

## Usage
- `echo "url=http://example.com" | git credential fill` : 获取凭证信息（从配置文件中读取用户名和密码）
- `echo "url=http://example.com" | git credential approve` : 存储凭证信息（保存到配置的凭证助手中）
- `echo "url=http://example.com" | git credential reject` : 删除凭证信息（从所有配置的凭证助手中清除）

## Links
- More information: https://git-scm.com/docs/git-credential
