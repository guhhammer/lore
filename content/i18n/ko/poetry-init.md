# poetry init

기본 `pyproject.toml` 파일을 대화형으로 생성.

## Usage
- `poetry init` : 대화형으로 `pyproject.toml` 파일 생성
- `poetry init --name 패키지_이름 --author "author_name <email@example.com>"` : 미리 지정한 값을 사용하여 `pyproject.toml` 파일 생성
- `poetry init -n` : 대화형 입력 없이 `pyproject.toml` 생성 (기본값을 사용)
- `poetry init --dependency 패키지_이름` : 의존성 추가하며 `pyproject.toml`파일 생성
- `poetry init --dev-dependency 패키지_이름` : 개발 의존성을 추가하며 `pyproject.toml` 파일 생성
- `poetry init -h` : 도움말 표시

## Links
- More information: https://python-poetry.org/docs/cli/#init
