# wezterm cli

실행중인 Wezterm GUI 또는 멀티플렉서와 상호작용.

## Usage
- `wezterm cli list` : 창, 탭 및 패널 목록 표시
- `wezterm cli split-pane --left|right|top|bottom --cells|percent n` : 현재 패널을 분할하고 새로운 패널의 ID를 `stdout`으로 출력
- `wezterm cli activate-pane --pane-id 아이디` : 지정한 패널 활성화 (포커스 이동)
- `wezterm cli kill-pane --pane-id 아이디` : 지정한 패널 종료

## Links
- More information: https://wezterm.org/cli/cli/index.html
