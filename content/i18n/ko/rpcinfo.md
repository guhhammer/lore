# rpcinfo

RPC 서버에 RPC 호출을 수행하고 결과를 보고합니다.

## Usage
- `rpcinfo` : localhost에 등록된 모든 RPC 서비스의 전체 테이블 표시
- `rpcinfo -s localhost` : localhost에 등록된 모든 RPC 서비스의 간결한 테이블 표시
- `rpcinfo -m` : localhost에서 rpcbind 작업의 통계 테이블 표시
- `rpcinfo -l 원격_nfs_서버_ip mountd 2` : 원격 NFS 공유에서 주어진 서비스 이름(mountd)과 버전 번호(2)의 항목 목록 표시
- `rpcinfo -d mountd 1` : 모든 전송 방식에 대해 mountd 서비스의 버전 1 등록 삭제

## Links
- More information: https://manned.org/rpcinfo
