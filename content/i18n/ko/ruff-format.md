# ruff format

매우 빠른 Python 코드 포매터. 파일이나 디렉토리가 지정되지 않으면 기본적으로 현재 작업 디렉토리가 사용됩니다.

## Usage
- `ruff format 경로/대상/파일_또는_폴더1 경로/대상/파일_또는_폴더2 ...` : 지정된 파일이나 디렉토리를 직접 포맷
- `ruff format --check` : 수정될 파일을 출력하고, 포맷이 필요한 파일이 있으면 0이 아닌 종료 코드를 반환하며, 그렇지 않으면 0을 반환
- `ruff format --diff` : 파일을 수정하지 않고 어떤 변경이 이루어질지 출력

## Links
- More information: https://docs.astral.sh/ruff/formatter/
