# diff-pdf

2개의 PDF를 비교.

## Usage
- `diff-pdf 경로/대상/a.pdf 경로/대상/b.pdf` : 반환 코드를 사용하여, 변경 사항을 나타내는 PDF를 비교 (`0` = 차이 없음, `1` = PDF가 다름)
- `diff-pdf --output-diff=경로/대상/diff.pdf 경로/대상/a.pdf 경로/대상/b.pdf` : PDF를 비교하여, 시각적으로 강조된 차이점이 있는 PDF를 출력
- `diff-pdf --view 경로/대상/a.pdf 경로/대상/b.pdf` : PDF를 비교하고, 간단한 GUI에서 차이점을 확인

## Links
- More information: https://github.com/vslavik/diff-pdf
