# jj tag delete

`jj` 저장소의 태그를 삭제.

## Usage
- `jj tag d 태그_이름` : 태그 삭제
- `jj tag d 태그1 태그2 ...` : 여러 태그 삭제
- `jj tag d "glob:v1.*"` : `glob` 패턴과 일치하는 태그 삭제
- `jj tag d "하위문자열:릴리스"` : 부분 문자열 패턴과 일치하는 태그 삭제
- `jj tag d "exact:v1.0.0"` : 정확한 이름으로 태그 삭제

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-tag-delete
