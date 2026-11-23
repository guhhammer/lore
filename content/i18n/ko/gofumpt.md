# gofumpt

Go 파일을 엄격한 규칙으로 포맷하는 도구.

## Usage
- `gofumpt -w 경로/대상/디렉터리` : go 파일 포맷 적용
- `gofumpt -l 경로/대상/디렉터리` : `gofumpt` 포맷과 다른 파일 목록(list) 표시
- `gofumpt -e 경로/대상/디렉터리` : 모든 오류(errors) 보고
- `gofumpt -d 경로/대상/디렉터리` : 변경 사항(diffs) 표시
- `gofumpt -extra 경로/대상/디렉터리` : 더욱 엄격한 규칙으로 Go 파일 포맷
- `gofumpt -extra -d 경로/대상/디렉터리` : 더욱 엄격한 규칙 기준으로 변경 사항(diffs) 표시
- `gofumpt -h` : 도움말 표시

## Links
- More information: https://github.com/mvdan/gofumpt#gofumpt
