# poetry update

`pyproject.toml` 파일에 따라 프로젝트 의존성을 업데이트.

## Usage
- `poetry update` : 모든 의존성 업데이트
- `poetry update 패키지1 패키지2 ...` : 지정한 하나 이상의 패키지 업데이트
- `poetry update --lock` : 패키지는 설치하지 않고 잠금 파일만 업데이트
- `poetry update --sync` : 잠금된 패키지에 맞게 환경 동기화
- `poetry update --only 그룹_이름` : 지정한 의존성 그룹만 업데이트
- `poetry update --dry-run` : 실제 변경 없이 업데이트 과정 시뮬레이션

## Links
- More information: https://python-poetry.org/docs/cli/#update
