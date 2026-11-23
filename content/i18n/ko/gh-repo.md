# gh repo

GitHub 저장소 작업 수행.

## Usage
- `gh repo new` : 새로운 저장소를 대화형으로 생성
- `gh repo clone 소유자/저장소` : 저장소 복제
- `gh repo fork 소유자/저장소 --clone` : 저장소를 fork 후 clone 수행
- `gh repo view 저장소 -w` : 기본 웹 브라우저에서 저장소 보기
- `gh repo ls 소유자` : 특정 사용자 또는 조직이 소유한 저장소 목록 표시 (소유자를 지정하지 않으면, 현재 로그인한 사용자 사용)
- `gh repo ls 소유자 --source -L 제한_개수` : fork가 아닌 저장소만 표시하고 목록 개수 제한 (기본값: 30)
- `gh repo ls 소유자 -l 언어_이름` : 특정 주 사용 언어를 가진 저장소 목록 표시
- `gh repo sync` : 현재 로컬 저장소를 upstream과 동기화

## Links
- More information: https://cli.github.com/manual/gh_repo
