# tidy-viewer

CSV, TSV, PSV, Parquet 및 Arrow IPC 파일을 열 스타일과 함께 보기 좋게 출력. 일반적으로 `tv`라는 별칭으로도 사용됨.

## Usage
- `tidy-viewer 경로/대상/파일.csv` : CSV 파일의 첫 25개 행을 보기 좋게 추력
- `cat 경로/대상/파일.csv | tidy-viewer` : `stdin`으로 전달된 데이터를 보기 좋게 출력
- `tidy-viewer -s '|' 경로/대상/파일.psv` : 사용자 지정 구분자(separator)를 사용해 파일 출력 (예: 파이프)
- `tidy-viewer -c 3 경로/대상/파일.csv` : 지정한 색상 팔레트로 데이터 표시 (1: nord, 2: one_dark, 3: gruvbox, 4: dracula, 5: solarized light)
- `tidy-viewer -af 경로/대상/파일.csv | less -RS` : 모든 행을 컬러로 출력하고, 페이지 단위로 타색
- `tidy-viewer -h` : 도움말 표시
- `tidy-viewer -V` : 버전 표시

## Links
- More information: https://github.com/alexhallam/tv#help
