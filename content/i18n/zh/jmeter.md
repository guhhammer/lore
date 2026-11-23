# jmeter

开源的 Java 应用程序，旨在对功能行为进行负载测试并衡量性能。

## Usage
- `jmeter -n -t 路径/到/文件.jmx` : 在非 GUI 模式下运行指定的测试计划
- `jmeter -n -t 路径/到/文件.jmx -l 路径/到/日志文件.jtl` : 在非 GUI 模式下使用指定的日志文件运行测试计划
- `jmeter -n -t 路径/到/文件.jmx -H- 127.0.0.1 -P 8888` : 在非 GUI 模式下使用指定代理运行测试计划
- `jmeter -n -t 路径/到/文件.jmx -J 键='值'` : 在非 GUI 模式下使用指定的 JMeter 属性运行测试计划

## Links
- More information: https://jmeter.apache.org/usermanual/get-started.html#options
