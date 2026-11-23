# sbcast

작업에 할당된 노드로 파일 전송. 이 명령은 Slurm 배치 작업 내에서만 사용해야 합니다.

## Usage
- `sbcast 경로/대상/파일 경로/대상/목적지` : 현재 작업에 할당된 모든 노드로 파일 전송
- `sbcast --send-libs=yes 경로/대상/실행파일 경로/대상/목적지` : 전송하는 파일이 의존하는 공유 라이브러리를 자동으로 감지하여 함께 전송

## Links
- More information: https://slurm.schedmd.com/sbcast.html
