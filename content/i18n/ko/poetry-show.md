# poetry show

Poetry 프로젝트의 패키지 상세 정보를 표시.

## Usage
- `poetry show` : 모든 패키지 표시
- `poetry show 패키지_이름` : 지정한 패키지의 상세 정보 표시
- `poetry show -t` : 의존성 트리 형식으로 상세 정보 표시
- `poetry show -T` : 최상위 패키지만 표시 (`pyproject.toml`에 명시적으로 정의된 패키지)
- `poetry show -o` : 오래된 패키지 표시
- `poetry show -l` : 모든 패키지의 최신 버전 표시
- `poetry show --without 그룹1,그룹2,...` : 지정한 의존성 그룹 제외
- `poetry show --only 그룹1,그룹2,...` : 지정한 의존성 그룹만 표시

## Links
- More information: https://python-poetry.org/docs/cli/#show
