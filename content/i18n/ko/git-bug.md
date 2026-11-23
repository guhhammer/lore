# git bug

Git의 내부 저장소를 사용하는 분산 버그 추적기입니다. 프로젝트에 파일이 추가되지 않습니다. 문제를 커밋 및 브랜치처럼 다른 사람들과 상호작용하는 동일한 Git 원격 저장소에 제출할 수 있습니다.

## Usage
- `git bug user create` : 새 사용자 생성
- `git bug add` : 새 버그 생성
- `git bug push` : 원격 저장소에 새로운 버그 항목 푸시
- `git bug pull` : 업데이트 가져오기
- `git bug ls` : 기존 버그 나열
- `git bug ls "상태:열림 정렬:편집"` : 쿼리를 사용하여 버그 필터링 및 정렬
- `git bug ls "검색_쿼리" baz` : 텍스트 내용으로 버그 검색

## Links
- More information: https://github.com/git-bug/git-bug/blob/master/doc/md/git-bug.md
