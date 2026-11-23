# jj simplify-parents

지정한 리비전의 부모 연결을 단순화. 예를 들어, "A -> B -> C | A -> C"가 함께 있는 경우, "A -> B -> C"만 남도록 단순화.

## Usage
- `jj simplify-parents -r revsets` : 지정한 리비전의 부모 연결 단순화
- `jj simplify-parents -s revsets` : 지정한 리비전과 그 모든 자식 리비전 트리의 부모 연결 단순화

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-simplify-parents
