# yapf

Python 스타일 가이드 검사기.

## Usage
- `yapf --diff 경로/대상/파일` : 변경 사항을 적용하지 않고 변경 사항의 차이를 표시 (드라이런)
- `yapf --diff --in-place 경로/대상/파일` : 파일을 제자리에서 포맷하고 변경 사항의 차이를 표시
- `yapf --recursive --in-place --style pep8 --parallel 경로/대상/폴더` : 디렉토리 내의 모든 Python 파일을 재귀적으로 동시에 포맷

## Links
- More information: https://github.com/google/yapf#usage
