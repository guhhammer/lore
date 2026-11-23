# git apply

将补丁应用到文件和/或暂存区，但不创建提交。

## Usage
- `git apply -v 路径/到/文件` : 显示补丁文件的应用详情
- `git apply --index 路径/到/文件` : 应用补丁并将修改添加到暂存区
- `curl -L https://example.com/file.patch | git apply` : 应用远程补丁文件
- `git apply --stat --apply 路径/到/文件` : 应用远程补丁文件
- `git apply -R 路径/到/文件` : 反向应用补丁（撤销更改）
- `git apply --cached 路径/到/文件` : 将补丁结果存入暂存区但不修改工作区

## Links
- More information: https://git-scm.com/docs/git-apply
