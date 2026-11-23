# repomix

Github 저장소를 AI가 처리하기 쉬운 하나의 파일로 패키징.

## Usage
- `repomix -o 경로/대상/파일 --style xml|markdown|plain` : 사용자 지정 형식으로 출력
- `repomix --stdout >경로/대상/파일` : 결과를 `stdout`으로 출력 후 파일에 저장
- `repomix --stdout | less` : 결과를 `stdout`으로 출력한 뒤, 다른 프로그램으로 전달
- `repomix --compress` : 압축하여 출력
- `repomix --include "src/**/*.ts" --ignore "**/*.test.ts"` : 지정한 파일만 포함하고 특정 파일은 제외하여 처리
- `repomix --remote https://github.com/user/repo/tree/main` : 지정한 브랜치의 저장소 패키징
- `repomix --remote https://github.com/user/repo/commit/836abcd7335137228ad77feb28655d85712680f1` : 지정한 커밋 시점의 저장소 패키징
- `repomix --remote 사용자/저장소` : 축약된 형식으로 저장소 패키징

## Links
- More information: https://github.com/yamadashy/repomix
