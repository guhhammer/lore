# showmount

NFS 서버의 마운트 정보를 표시.

## Usage
- `showmount 호스트명` : 현재 서버에서 마운트 중인 클라이언트를 표시
- `showmount -e 호스트명` : NFS 서버의 export 목록을 표시
- `showmount -a 호스트명` : 모든 클라이언트와 각 클라이언트가 마운트한 디렉터리를 표시
- `showmount -d 호스트명` : 클라이언트가 마운트한 디렉터리만 표시
- `showmount -e --no-headers 호스트명` : 헤더 없이 export 목록을 표시
- `showmount -h` : 도움말 표시

## Links
- More information: https://manned.org/showmount
