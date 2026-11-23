# git branch

Git 브랜치 작업을 위한 주요 명령어.

## Usage
- `git branch -a` : 모든 브랜치 나열 (로컬 및 원격; 현재 브랜치는 `*`로 강조)
- `git branch -a --contains 커밋_해시` : 특정 Git 커밋을 포함한 브랜치 나열
- `git branch --show-current` : 현재 브랜치 이름 표시
- `git branch 브랜치_이름` : 현재 커밋을 기준으로 새 브랜치 생성
- `git branch 브랜치_이름 커밋_해시` : 특정 커밋을 기준으로 새 브랜치 생성
- `git branch -m 이전_브랜치_이름 새_브랜치_이름` : 브랜치 이름 변경 (현재 체크아웃된 브랜치가 아니어야 함)
- `git branch -d 브랜치_이름` : 로컬 브랜치 삭제 (현재 체크아웃된 브랜치가 아니어야 함)
- `git push 원격_이름 -d 원격_브랜치_이름` : 원격 브랜치 삭제

## Links
- More information: https://git-scm.com/docs/git-branch
