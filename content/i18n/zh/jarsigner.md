# jarsigner

签名并验证 Java 存档（`.jar`）文件。

## Usage
- `jarsigner path/to/file.jar keystore_alias` : 签名一个 `.jar` 文件
- `jarsigner -sigalg algorithm path/to/file.jar keystore_alias` : 使用特定算法对 `.jar` 文件进行签名
- `jarsigner -verify path/to/file.jar` : 验证 `.jar` 文件的签名

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/jarsigner.html
