# git am

应用补丁文件并创建提交。在通过电子邮件接收提交时非常有用。

## Usage
- `git am 路径/到/目录.patch` : 应用本地补丁文件并提交更改
- `curl -L https://example.com/file.patch | git am` : 应用远程补丁文件并提交更改
- `git am --abort` : 中止应用补丁文件的过程
- `git am --reject 路径/到/目录.patch` : 尽可能应用补丁文件，将失败的代码块保存到拒绝文件中

## Links
- More information: https://git-scm.com/docs/git-am
