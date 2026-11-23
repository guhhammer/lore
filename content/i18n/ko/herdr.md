# herdr

AI 코딩 에이전트를 위한 터미널 작업 공간 관리자. 참고: 레이아웃은 Session > Workspace > Tab > Pane 구조로 구성됨.

## Usage
- `herdr` : 새로운 세션 시작 또는 기본 세션에 연결
- `herdr status` : 로컬 클라이언트 및 서버 상태를 표시
- `herdr --default-config` : 기본 설정을 생성해 `stdout`으로 출력
- `<Ctrl b><q>` : 현재 세션에서 분리 (herdr 세션 내부)
- `<Ctrl b><?>` : 키 바인딩 표시 (herdr 세션 내부)
- `<Ctrl b><Shift n>` : 새로운 작업 공간 생성 (herdr 세션 내부)
- `<Ctrl b><c>` : 새로운 탭 생성 (herdr 세션 내부)
- `<Ctrl b><v>|<->` : 패널을 수직/수평으로 분할 (herdr 세션 내부)

## Links
- More information: https://herdr.dev/docs/cli-reference/
