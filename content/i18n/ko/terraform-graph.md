# terraform graph

설정 또는 실행 계획의 시각적 표현을 생성합니다. 출력은 DOT 형식이며 GraphViz를 사용하여 렌더링할 수 있습니다.

## Usage
- `terraform graph` : 현재 구성에 대한 종속성 그래프 생성
- `terraform graph | dot -Tpng >경로/대상/그래프.png` : 그래프를 생성하고 GraphViz를 사용하여 PNG 파일로 렌더링
- `terraform graph -plan=경로/대상/tfplan` : 저장된 계획 파일에서 그래프 생성
- `terraform graph -draw-cycles` : 종속성 주기를 강조하는 그래프 생성
- `terraform graph -type=plan|plan-refresh-only|plan-destroy|apply` : 특정 유형의 그래프 생성 (plan, apply 등)

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/graph
