# pueue enqueue

저장된 작업을 대기열에 추가.

## Usage
- `pueue enqueue 작업_아이디 작업_아이디` : 여러 저장된 작업을 한 번에 대기열에 추가
- `pueue enqueue -d 60 작업_아이디` : 60초 후에 저장된 작업을 대기열에 추가
- `pueue enqueue -d wednesday 작업_아이디` : 다음 수요일에 저장된 작업을 대기열에 추가
- `pueue enqueue -d "4 months" 작업_아이디` : 4개월 후에 저장된 작업을 대기열에 추가
- `pueue enqueue -d 2021-02-19 작업_아이디` : 2021-02-19에 저장된 작업을 대기열에 추가
- `pueue enqueue -h` : 사용 가능한 모든 날짜/시간 형식 나열

## Links
- More information: https://github.com/Nukesor/pueue#how-to-use-it
