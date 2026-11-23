# gh ruleset

GitHub 저장소 규칙모음 관리.

## Usage
- `gh rs ls` : 현재 저장소의 모든 규칙모음 목록 표시
- `gh rs ls -o 조직_이름` : 특정 조직의 모든 규칙모음 목록 표시
- `gh rs check` : 현재 브랜치에 적용되는 규칙 확인
- `gh rs check 브랜치_이름 -R 소유자/저장소` : 다른 저장소의 특정 브랜치에 적용되는 규칙 확인
- `gh rs view` : 현재 저장소의 규칙모음을 대화형으로 선택하여 보기
- `gh rs view 규칙모음_아이디` : ID로 특정 규칙모음 보기
- `gh rs view 규칙모음_아이디 -o 조직_이름` : ID로 조직 수준 규칙모음 보기
- `gh rs ls -R 소유자/저장소 -w` : 특정 저장소의 규칙모음 목록을 브라우저에서 열기

## Links
- More information: https://cli.github.com/manual/gh_ruleset
