# uv

빠른 Python 패키지 및 프로젝트 관리자. `tool` 및 `python`과 같은 일부 하위 명령에는 자체 사용 설명서가 있습니다.

## Usage
- `uv init` : 현재 디렉토리에 새 Python 프로젝트 생성
- `uv init 프로젝트_이름` : 주어진 이름의 디렉토리에 새 Python 프로젝트 생성
- `uv add 패키지` : 프로젝트에 새 패키지 추가
- `uv remove 패키지` : 프로젝트에서 패키지 제거
- `uv run 경로/대상/스크립트.py` : 프로젝트 환경에서 스크립트 실행
- `uv run 명령` : 프로젝트 환경에서 명령 실행
- `uv sync` : `pyproject.toml`에서 프로젝트 환경 업데이트
- `uv lock` : 프로젝트의 의존성에 대한 lock 파일 생성

## Links
- More information: https://docs.astral.sh/uv/reference/cli/
