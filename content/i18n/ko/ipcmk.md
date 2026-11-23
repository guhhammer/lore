# ipcmk

IPC(프로세스 간 통신) 리소스 생성.

## Usage
- `ipcmk -M 세그먼트_크기_바이트` : 공유 메모리 세그먼트 생성
- `ipcmk -S 요소_크기` : 세마포어 생성
- `ipcmk -Q` : 메시지 큐 생성
- `ipcmk -M 세그먼트_크기_바이트 8진수_권한` : 특정 권한으로 공유 메모리 세그먼트 생성 (기본값은 0644)

## Links
- More information: https://manned.org/ipcmk
