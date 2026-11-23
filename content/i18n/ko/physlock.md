# physlock

모든 콘솔 및 가상 터미널을 잠급니다.

## Usage
- `physlock` : 모든 콘솔 잠금 (해제하려면 현재 사용자 또는 root 필요)
- `physlock -m` : 잠금 동안 콘솔의 커널 메시지 음소거
- `physlock -s` : 잠금 동안 SysRq 메커니즘 비활성화
- `physlock -p "잠겼습니다!"` : 암호 입력 전 메시지 표시
- `physlock -d` : physlock을 포크하고 분리 (일시 중지 또는 최대 절전 모드 스크립트에 유용)

## Links
- More information: https://github.com/xyb3rt/physlock#usage
