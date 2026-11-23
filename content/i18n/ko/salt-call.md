# salt-call

로컬에서 salt minion에서 salt를 호출합니다.

## Usage
- `salt-call state.highstate` : 이 minion에서 highstate 실행
- `salt-call state.highstate test=true` : highstate 시뮬레이션 실행, 모든 변경 사항을 계산하지만 실제로 수행하지 않음
- `salt-call -l debug state.highstate` : 자세한 디버깅 출력과 함께 highstate 실행
- `salt-call grains.items` : 이 minion의 grains 나열

## Links
- More information: https://docs.saltproject.io/en/latest/ref/cli/salt-call.html
