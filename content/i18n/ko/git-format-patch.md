# git format-patch

`.patch` 파일 준비. 커밋을 이메일로 전송할 때 유용합니다.

## Usage
- `git format-patch origin` : 푸시되지 않은 모든 커밋에 대한 자동 이름 지정 `.patch` 파일 생성
- `git format-patch revision_1..revision_2` : 두 개의 리비전 사이의 모든 커밋에 대한 `.patch` 파일을 `stdout`으로 출력
- `git format-patch -3` : 최근 3개의 커밋에 대한 `.patch` 파일 생성

## Links
- More information: https://git-scm.com/docs/git-format-patch
