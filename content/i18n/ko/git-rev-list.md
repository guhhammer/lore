# git rev-list

리비전(커밋)을 역순으로 나열합니다.

## Usage
- `git rev-list HEAD` : 현재 브랜치의 모든 커밋 나열
- `git rev-list -n 1 HEAD -- 경로/대상/파일` : 특정 파일이 변경(추가/편집/제거)된 최신 커밋 출력
- `git rev-list --since "2019-12-01 00:00:00" 브랜치_이름` : 특정 날짜 이후의 커밋을 특정 브랜치에서 나열
- `git rev-list --merges 커밋` : 특정 커밋의 모든 병합 커밋 나열
- `git rev-list 태그_이름..HEAD --count` : 특정 태그 이후의 커밋 수 출력

## Links
- More information: https://git-scm.com/docs/git-rev-list
