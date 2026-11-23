# poetry cache

Poetry 캐시를 관리.

## Usage
- `poetry cache list` : 사용 가능한 Poetry 캐시 목록 표시
- `poetry cache clear PyPI --all` : 지정한 캐시에서 모든 패키지 제거 (예: PyPI)
- `poetry cache clear pypi:requests:2.24.0` : 캐시에서 지정한 패키지 제거 (참고: `cache:package:version` 형식으로 지정)

## Links
- More information: https://python-poetry.org/docs/cli/#cache
