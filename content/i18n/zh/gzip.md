# gzip

使用 `gzip` (LZ77) 压缩/解压文件。

## Usage
- `gzip 路径/到/文件` : 压缩文件，将其替换为 `gzip` 存档文件
- `gzip -d 路径/到/文件.gz` : 解压缩文件，将其替换为原始未压缩版本
- `gzip -k 路径/到/文件` : 压缩文件，保留原始文件
- `gzip -c 路径/到/文件 >路径/到/压缩文件.gz` : 压缩文件，指定输出文件名
- `gzip -c -d 路径/到/文件.gz >路径/到/解压缩的文件名` : 解压缩 `gzip` 存档，指定输出文件名
- `gzip -1..9 -c 路径/到/文件 >路径/到/压缩文件.gz` : 指定压缩级别。 1 为最快（低压缩率），9 为最慢（高压缩率），默认值是 6
- `gzip -v -d 路径/到/文件.gz` : 显示压缩或解压后的每个文件的名称和压缩百分比

## Links
- More information: https://www.gnu.org/software/gzip/manual/gzip.html
