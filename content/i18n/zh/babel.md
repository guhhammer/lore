# babel

一款 JavaScript 的编译器，将下一代 ES 语法转换为兼容语法。

## Usage
- `babel 路径/到/文件` : 转编译指定文件到标准输出
- `babel 路径/到/输入文件 --out-file 路径/到/输出文件` : 转编译指定文件，输入为特定文件
- `babel 路径/到/输入文件 --watch` : 监听文件变动触发转编译
- `babel 路径/到/输入文件目录` : 转编译整个目录下的 js 文件
- `babel 路径/到/输入文件目录 --ignore 被忽略文件` : 跳过指定目录下指定文件的编译（多文件使用英文逗号“,”分隔）
- `babel 路径/到/输入文件 --minified` : 转编译后，执行压缩
- `babel 路径/到/输入文件 --presets 预设项` : 使用预设值
- `babel --help` : 显示帮助

## Links
- More information: https://babeljs.io/docs/babel-cli
