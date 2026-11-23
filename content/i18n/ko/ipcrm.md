# ipcrm

IPC(프로세스 간 통신) 리소스를 삭제합니다.

## Usage
- `ipcrm -m 공유_메모리_id` : ID로 공유 메모리 세그먼트 삭제
- `ipcrm -M 공유_메모리_키` : 키로 공유 메모리 세그먼트 삭제
- `ipcrm -q ipc_큐_id` : ID로 IPC 큐 삭제
- `ipcrm -Q ipc_큐_키` : 키로 IPC 큐 삭제
- `ipcrm -s 세마포어_id` : ID로 세마포어 삭제
- `ipcrm -S 세마포어_키` : 키로 세마포어 삭제
- `ipcrm -a` : 모든 IPC 리소스 삭제

## Links
- More information: https://manned.org/ipcrm
