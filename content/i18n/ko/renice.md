# renice

실행 중인 프로세스의 스케줄링 우선순위/친화도를 변경. 친화도 값은 -20(프로세스에 가장 유리함)에서 19(프로세스에 가장 불리함)까지.

## Usage
- `renice -n 3 -p pid` : 실행 중인 process의 우선순위를 증가/감소
- `renice -n -4 -u uid|사용자` : 특정 user가 소유한 모든 프로세스의 우선순위를 증가/감소
- `renice -n 5 -g 프로세스_그룹` : 프로세스 group에 속한 모든 프로세스의 우선순위를 증가/감소

## Links
- More information: https://manned.org/renice.1p
