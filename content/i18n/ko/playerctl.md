# playerctl

MPRIS를 통해 미디어 플레이어 제어.

## Usage
- `playerctl play-pause` : 재생/일시정지 전환
- `playerctl next` : 다음 트랙으로 건너뛰기
- `playerctl previous` : 이전 트랙으로 돌아가기
- `playerctl -l` : 모든 플레이어 나열
- `playerctl -p 플레이어_이름 play-pause|next|previous|...` : 특정 플레이어에 명령 전송
- `playerctl -a play-pause|next|previous|...` : 모든 플레이어에 명령 전송
- `playerctl metadata -f "현재 재생 중: \{\{artist\}\} - \{\{album\}\} - \{\{title\}\}"` : 현재 트랙에 대한 메타데이터 표시

## Links
- More information: https://github.com/altdesktop/playerctl#using-the-cli
