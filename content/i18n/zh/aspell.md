# aspell

交互式拼写检查工具。 更多信息：<http://aspell.net/man-html/index.html>。

## Usage
- `aspell check 文件路径` : 为一个文件做拼写检查
- `cat 文件 | aspell list` : 列出来自标准输入的拼写错误单词
- `aspell dicts` : 列出可用的字典语言
- `aspell --lang cs` : 指定不同的语言（取 ISO 639 语言代码的 2 个字母）来运行 `aspell`
- `cat 文件 | aspell --personal 个人单词列表.pws list` : 列出来自标准输入的拼写错误单词，并且忽略个人单词列表中的单词
