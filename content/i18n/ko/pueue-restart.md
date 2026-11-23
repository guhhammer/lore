# pueue restart

작업을 다시 시작.

## Usage
- `pueue restart 작업_아이디` : 특정 작업 다시 시작
- `pueue restart -k 작업_아이디 작업_아이디` : 여러 작업을 한 번에 다시 시작하고 즉시 시작 (대기열에 넣지 않음)
- `pueue restart --edit-path 작업_아이디` : 다른 경로에서 특정 작업 다시 시작
- `pueue restart -e 작업_아이디` : 다시 시작하기 전에 명령 편집
- `pueue restart -i 작업_아이디` : 작업을 제자리에서 다시 시작 (별도의 작업으로 대기열에 넣지 않음)
- `pueue restart -a --stashed` : 실패한 모든 작업 다시 시작 및 저장

## Links
- More information: https://github.com/Nukesor/pueue#how-to-use-it
