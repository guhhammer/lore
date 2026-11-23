# cliphist

Wayland 컴포지터의 클립보드 기록을 관리. `wl-copy` 및 `wl-paste`와 함께 사용됨.

## Usage
- `cliphist list` : 클립보드 기록 목록 표시
- `cliphist list | fzf | cliphist decode | wl-copy` : 이전 클립보드 항목을 선택하여 다시 복사 (`fzf` 사용)
- `cliphist wipe` : 저장된 모든 클립보드 기록 삭제
- `echo 아이디 | cliphist delete` : 지정한 ID의 클립보드 기록 삭제
- `wl-paste | cliphist store` : 현재 클립보드 내용을 수동으로 저장

## Links
- More information: https://github.com/sentriz/cliphist#usage
