# mvn

Apache Maven：构建和管理基于 Java 的项目。

## Usage
- `mvn compile` : 编译项目
- `mvn package` : 将编译后的代码打包成可分发格式，比如 `jar`
- `mvn package -D skipTests` : 编译和打包，跳过单元测试
- `mvn install` : 在本地 maven 存储库中安装构建的包（这也会调用 compile 和 package 命令）
- `mvn clean` : 从目标目录中删除构建工件，通常用来清理之前的编译结果
- `mvn clean package` : 执行清理操作，然后进行编译打包
- `mvn clean -P 构建配置文件 package` : 清理然后使用给定的构建配置文件打包代码
- `mvn exec:java -D exec.mainClass="com.example.Main" -D exec.args="参数1 参数2 ..."` : 使用 main 方法运行一个类

## Links
- More information: https://manned.org/mvn
