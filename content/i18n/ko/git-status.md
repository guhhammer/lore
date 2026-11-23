# git status

Git 저장소의 파일 변경 사항을 표시합니다. 현재 체크아웃된 커밋과 비교하여 변경된, 추가된 및 삭제된 파일을 나열합니다.

## Usage
- `git status` : 추적되지 않는 파일과 아직 스테이징하지 않은 변경 파일 보기
- `git status -s` : short 형식으로 출력
- `git status -sb` : short 형식으로 출력하면서 branch 정보 표시
- `git status -v` : 스테이징된 파일의 변경 사항 표시 (`git diff --cached`와 유사)
- `git status -vv` : 추적 중인 모든 파일의 변경 사항 표시 (`git diff HEAD`와 유사)
- `git status --show-stash` : 현재 숨겨둔 엔트리의 수 표시
- `git status -uno` : 출력에 추적되지 않는 파일을 표시하지 않기

## Links
- More information: https://git-scm.com/docs/git-status
