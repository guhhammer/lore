# git cat-file

Git 저장소 객체의 콘텐츠 또는 유형 및 크기 정보를 제공합니다.

## Usage
- `git cat-file -s HEAD` : HEAD 커밋의 크기(바이트 단위) 확인
- `git cat-file -t 8c442dc3` : 주어진 Git 객체의 유형(blob, tree, commit, tag) 확인
- `git cat-file -p HEAD~2` : 주어진 Git 객체의 유형에 따라 콘텐츠를 보기 좋게 출력

## Links
- More information: https://git-scm.com/docs/git-cat-file
