# qtile

Python으로 작성 및 구성된 완전한 기능의 해킹 가능한 타일링 윈도우 매니저.

## Usage
- `qtile start` : 윈도우 매니저를 시작 (이미 실행 중이 아니라면, `.xsession` 또는 유사한 곳에서 실행하는 것이 이상적)
- `qtile check` : 설정 파일에 컴파일 오류가 있는지 확인 (기본 위치는 `~/.config/qtile/config.py`)
- `qtile top --force` : 현재 리소스 사용 정보 표시
- `qtile run-cmd --group test-group --float xterm` : `test-group`이라는 그룹에서 `xterm` 프로그램을 플로팅 윈도우로 열기
- `qtile cmd-obj --object cmd --function restart` : 윈도우 매니저 다시 시작

## Links
- More information: https://docs.qtile.org/en/latest/manual/commands/shell/index.html
