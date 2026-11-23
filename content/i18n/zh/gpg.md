# gpg

GNU Privacy Guard。

## Usage
- `gpg --full-gen-key` : 交互地创建 GPG 公钥和私钥
- `gpg --clearsign doc.txt` : 不加密，仅对 `doc.txt` 进行签名（生成 `doc.txt.asc`，格式为 ASCII 码形式）
- `gpg -es -r alice@example.com -r bob@example.com doc.txt` : 为接收者 alice@example.com 和 bob@example.com 签名并加密 `doc.txt`（生成 `doc.txt.gpg`）
- `gpg -c doc.txt` : 只用密码加密 `doc.txt`（生成 `doc.txt.gpg`）
- `gpg -d doc.txt.gpg` : 解密 `doc.txt.gpg`（输出到标准输出）
- `gpg --import public.gpg` : 导入一个公钥
- `gpg --export -a alice@example.com` : 导出 alice@example.com 的公钥（输出到标准输出）
- `gpg --export-secret-keys -a alice@example.com` : 导出 alice@example.com 的私钥（输出到标准输出）

## Links
- More information: https://gnupg.org/documentation/manuals/gnupg/Invoking-GPG.html
