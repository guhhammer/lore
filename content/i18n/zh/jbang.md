# jbang

简便地创建、编辑和运行仅包含源代码的自包含 Java 程序。

## Usage
- `jbang init 路径/到/文件.java` : 初始化一个简单的 Java 类
- `jbang init -t=cli 路径/到/文件.java` : 初始化一个 Java 类（用于脚本编写）
- `jbang run -i` : 使用 `jshell` 在 REPL 编辑器中探索和使用脚本及其任何依赖项
- `jbang edit --open=codium|code|eclipse|idea|netbeans|gitpod 路径/到/脚本.java` : 设置一个临时项目以在 IDE 中编辑脚本
- `echo 'Files.list(Paths.get("/etc")).forEach(System.out::println);' | jbang -` : 运行 Java 代码片段（Java 9 及以后版本）
- `jbang 路径/到/文件.java 命令 参数1 参数2 ...` : 运行命令行应用程序
- `jbang app install --name 命令名称 路径/到/脚本.java` : 在用户的 `$PATH` 中安装一个脚本
- `jbang jdk install 版本` : 安装一个特定版本的 JDK 以与 `jbang` 一起使用

## Links
- More information: https://www.jbang.dev/documentation/jbang/latest/cli/jbang.html
