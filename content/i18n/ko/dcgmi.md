# dcgmi

NVIDIA Data Center GPU를 관리하고 모니터링.

## Usage
- `dcgmi discovery -l` : 사용 가능한 모든 GPU와 이를 사용하는 프로세스 정보를 표시
- `dcgmi group -l` : 생성된 그룹 목록을 표시
- `dcgmi dmon -e1001,1002,1003,1004,1005` : 디바이스 0의 현재 사용 통계를 표시
- `dcgmi -h` : 도움말 표시
- `dcgmi 하위명령어 -h` : 하위 명령어의 도움말을 표시

## Links
- More information: https://developer.nvidia.com/dcgm
