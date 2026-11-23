# arthas-trace

메서드 호출 체인을 추적하고, 경로상의 각 노드에 대한 소요 시간을 출력.

## Usage
- `trace class_pattern method_pattern` : 메서드 호출 체인을 추적
- `trace class_pattern method_pattern '#cost > 10'` : 메서드 호출 체인을 추적하고, 호출 시간이 10ms보다 긴 경우만 표시
- `trace -E class_pattern1|class_pattern2 method_pattern1|method_pattern2|method_pattern3` : 여러 클래스 또는 여러 메서드의 호출 체인을 추적
- `trace class_pattern method_pattern '#cost > 10' -n 5` : 메서드 호출 체인을 추적하고, 호출 시간이 10ms를 초과하는 경우만 표시하며, 5회 추적 후 중료

## Links
- More information: https://arthas.aliyun.com/en/doc/trace.html
