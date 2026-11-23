# poetry export

Poetry의 잠금 파일을 다른 형식으로 내보냄. Poetry Export Plugin 제공하는 기능.

## Usage
- `poetry export -o requirements.txt` : 의존성을 `requirements.txt` 파일로 내보내기
- `poetry export -o requirements-dev.txt --dev` : 개발 의존성을 포함하여 의존성을 내보내기
- `poetry export -o requirements.txt --without-hashes` : 해시를 제외하고 의존성을 내보내기
- `poetry export -o requirements.txt -f requirements.txt` : 지정한 형식으로 의존성을 내보내기
- `poetry export -o requirements.txt --only main` : 지정한 의존성 그룹만 내보내기
- `poetry export -h` : 도움말 표시

## Links
- More information: https://github.com/python-poetry/poetry-plugin-export#usage
