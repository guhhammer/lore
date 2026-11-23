# sd

직관적인 찾기 및 바꾸기 도구.

## Usage
- `echo 'lorem ipsum 23   ' | sd '\s+$' ''` : `regex`를 사용하여 공백 제거 (출력 스트림: `stdout`)
- `echo 'cargo +nightly watch' | sd '(\w+)\s+\+(\w+)\s+(\w+)' 'cmd: $1, channel: $2, subcmd: $3'` : 캡처 그룹을 사용하여 단어 바꾸기 (출력 스트림: `stdout`)
- `sd -p 'window.fetch' 'fetch' 경로/대상/파일.js` : 특정 파일에서 찾기 및 바꾸기 (출력 스트림: `stdout`)
- `find . -type f -exec sd 'from "react"' 'from "preact"' {} \;` : 현재 프로젝트의 모든 파일에서 찾기 및 바꾸기

## Links
- More information: https://manned.org/sd
