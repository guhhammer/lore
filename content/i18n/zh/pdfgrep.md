# pdfgrep

在 PDF 文件中搜索文本。

## Usage
- `pdfgrep 关键词 文件.pdf` : 在 PDF 中查找与关键词匹配的行
- `pdfgrep -H -n 关键词 文件.pdf` : 包含每个匹配行的文件名和页码
- `pdfgrep -m 3 -i '^foo' 文件.pdf` : 对以 "foo" 开头关键词搜索，返回前 3 个匹配项，不区分大小写
- `pdfgrep -r 关键词` : 在当前目录中扩展名为.pdf 的文件中递归查找关键词
- `pdfgrep -r --include '*book.pdf' 关键词` : 在与当前目录中特定文件名 "*book.pdf" 匹配的文件上递归查找关键词

## Links
- More information: https://pdfgrep.org/doc.html
