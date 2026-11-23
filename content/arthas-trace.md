# arthas-trace

Trace method invoke chain, and output the time cost for each node in the path.

## Usage
- `trace class_pattern method_pattern` : trace method invoke chain
- `trace class_pattern method_pattern '#cost > 10'` : trace method invoke chains and only display invoke information longer than 10 ms
- `trace -E class_pattern1|class_pattern2 method_pattern1|method_pattern2|method_pattern3` : trace the invoke chain of multiple classes or multiple methods
- `trace class_pattern method_pattern '#cost > 10' -n 5` : track method invoke chains, only display invoke information that exceeds 10 ms, and exit after 5 times

## See also
- arthas
- arthas-watch

## Categories
files

## Links
- More information: https://arthas.aliyun.com/en/doc/trace.html
- tldr page: https://tldr.inbrowser.app/pages/common/arthas-trace
