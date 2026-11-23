# git filter-branch

파일 제거 등 Git 브랜치 히스토리 변경.

## Usage
- `git filter-branch --tree-filter 'rm -f 파일' HEAD` : 모든 커밋에서 특정 파일 제거
- `git filter-branch --env-filter 'GIT_AUTHOR_EMAIL=새로운_이메일' HEAD` : 작성자 이메일 업데이트
- `git filter-branch --tree-filter 'rm -rf 폴더' HEAD` : 히스토리에서 특정 폴더 삭제

## Links
- More information: https://git-scm.com/docs/git-filter-branch
