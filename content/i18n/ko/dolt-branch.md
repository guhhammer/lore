# dolt branch

Dolt 브랜치 관리.

## Usage
- `dolt branch` : 로컬 분기 나열 (현재 분기는 `*`로 강조 표시됨)
- `dolt branch -A` : 모든 로컬 및 원격 브랜치를 나열
- `dolt branch 브랜치_이름` : 현재 브랜치를 기반으로 새로운 브랜치를 생성
- `dolt branch 브랜치_이름 커밋` : 지정된 커밋을 최신으로 사용하여 새로운 브랜치를 생성
- `dolt branch -m 브랜치_이름1 브랜치_이름2` : 브랜치 이름 변경
- `dolt branch -c 브랜치_이름1 브랜치_이름2` : 브랜치 복제
- `dolt branch -d 브랜치_이름` : 브랜치 삭제
- `dolt branch --show-current` : 현재 브랜치의 이름을 표시

## Links
- More information: https://docs.dolthub.com/cli-reference/cli#dolt-branch
