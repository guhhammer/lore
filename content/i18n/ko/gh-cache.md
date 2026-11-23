# gh cache

GitHub Actions 캐시 관리.

## Usage
- `gh cache ls` : 현재 저장소의 캐시 목록 표시
- `gh cache ls -R 소유자/저장소` : 특정 저장소의 캐시 목록 표시
- `gh cache ls -k 키_접두사` : 특정 캐시 키 접두사를 가진 캐시 목록 표시
- `gh cache ls -r refs/heads/브랜치_이름` : 특정 브랜치의 캐시 목록 표시
- `gh cache ls -S last_accessed_at -O asc` : 마지막 접근 시간이 가장 오래된 순으로 캐시 목록 정렬
- `gh cache delete 캐시_아이디` : ID로 캐시 삭제
- `gh cache delete 캐시_키` : 키로 캐시 삭제
- `gh cache delete -a` : 모든 캐시 삭제

## Links
- More information: https://cli.github.com/manual/gh_cache
