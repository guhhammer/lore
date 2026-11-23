# uv tree

프로젝트 의존성을 트리 형식으로 표시.

## Usage
- `uv tree` : 현재 환경의 의존성 트리 표시
- `uv tree --universal` : 모든 환경을 고려한 의존성 트리 표시
- `uv tree -d n` : 지정한 깊이까지 의존성 트리 표시
- `uv tree --outdated` : 오래된 모든 패키지의 사용 가능한 최신 버전 표시
- `uv tree --no-dev` : dev 그룹의 의존성을 제외하고 표시
- `uv tree --invert` : 트리 반전하여, 각 패키지에 의존하는 항목을 자식으로 표시

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-tree
