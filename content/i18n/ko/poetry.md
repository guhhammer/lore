# poetry

Python 패키지 및 의존성을 관리.

## Usage
- `poetry new 프로젝트_이름` : 특정 이름으로 디렉토리에 새 Poetry 프로젝트 생성
- `poetry add 의존성` : 현재 디렉토리의 `pyproject.toml` 파일에 의존성과 그 하위 의존성 설치 및 추가
- `poetry install` : 현재 디렉토리의 `pyproject.toml` 파일을 사용하여 프로젝트 의존성 설치
- `poetry init` : 현재 디렉토리를 새 Poetry 프로젝트로 상호작용 방식으로 초기화
- `poetry update` : 모든 의존성의 최신 버전 가져오고 `poetry.lock` 업데이트
- `poetry run 명령어` : 프로젝트의 가상 환경 내에서 명령어 실행
- `poetry version patch|minor|major|prepatch|preminor|premajor|prerelease` : `pyproject.toml`에서 프로젝트의 버전 증가
- `poetry shell` : 프로젝트의 가상 환경 내에서 셸 시작

## Links
- More information: https://python-poetry.org/docs/cli/
