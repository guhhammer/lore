# sccache

빠른 C/C++/Rust 컴파일러 캐시. 클라이언트와 서버로 구성되며, 둘 다 로컬 시스템에서 실행됨.

## Usage
- `sccache -s` : 컴파일 통계 표시
- `sccache gcc 경로/대상/파일.c` : `sccache`를 통해 `gcc` (또는 다른 컴파일러 명령) 실행
- `sccache --stop-server; SCCACHE_LOG=trace SCCACHE_START_SERVER=1 SCCACHE_NO_DAEMON=1 sccache` : `sccache` 서버를 포그라운드에서 시작하고 로그를 출력
- `sccache --dist-status` : 스케줄러에 분산 컴파일 상태 요청

## Links
- More information: https://manned.org/sccache
