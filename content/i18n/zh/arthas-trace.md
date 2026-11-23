# arthas-trace

方法内部调用路径，并输出方法路径上的每个节点上耗时。

## Usage
- `trace class_pattern method_pattern` : 追踪方法调用链
- `trace class_pattern method_pattern '#cost > 10'` : 追踪方法调用链，仅显示大于 10 毫秒的调用
- `trace -E class_pattern1|class_pattern2 method_pattern1|method_pattern2|method_pattern3` : 追踪多个类和方法的调用
- `trace class_pattern method_pattern '#cost > 10' -n 5` : 仅显示大于 10 毫秒的调用链，观测 10 次

## Links
- More information: https://arthas.aliyun.com/en/doc/trace.html
