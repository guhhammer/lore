# cancel

프린트 작업 취소.

## Usage
- `cancel` : 기본 프린터의 현재 작업을 취소 (`lpoptions -d printer`로 설정)
- `cancel -u 사용자명` : 특정 사용자(user)가 소유한 기본 프린터의 작업을 취소
- `cancel 프린터` : 특정 프린터의 현재 작업을 취소
- `cancel 프린터-작업_아이디` : 특정 프린터에서 특정 작업을 취소
- `cancel -a` : 모든 프린터의 모든(all) 작업을 취소
- `cancel -a 프린터` : 특정 프린터의 모든(all) 작업을 취소
- `cancel -h 서버 -x` : 특정 서버의 현재 작업을 취소한 다음 작업 데이터 파일을 삭제(x)

## Links
- More information: https://openprinting.github.io/cups/doc/man-cancel.html
