# ipcs

XSI IPC 자원(공유 메모리 세그먼트, 메시지 큐, 세마포어 등) 사용 정보에 대한 정보 표시.

## Usage
- `ipcs -a` : 모든(all) IPC 정보 출력
- `ipcs -m|-q|-s` : 활성 공유 메모리(memory), 메시지 큐(queues) 또는 세마포어(semaphore) 정보 출력
- `ipcs -b` : 최대 허용 크기(bytes) 정보 출력
- `ipcs -c` : 모든 IPC 자원의 생성자(creator's) 사용자 및 그룹 정보 출력
- `ipcs -p` : 모든 IPC 자원에 대해 마지막 작업을 수행한 프로세스의 PID(pID) 표시
- `ipcs -t` : 모든 IPC 자원 접근 시간(times) 정보 출력
- `ipcs -o` : 활성 메시지 큐, 공유 메모리의 현재(outstanding) 사용량 출력

## Links
- More information: https://manned.org/ipcs.1p
