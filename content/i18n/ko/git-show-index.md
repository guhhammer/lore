# git show-index

Git 저장소의 패키지된 아카이브 색인 표시.

## Usage
- `git show-index 경로/대상/파일.idx` : git 패키지 파일의 IDX 파일을 읽고 내용을 `stdout`에 덤프
- `git show-index --object-format sha1|sha256 경로/대상/파일` : 색인 파일의 해시 알고리즘 지정 (실험적)

## Links
- More information: https://git-scm.com/docs/git-show-index
