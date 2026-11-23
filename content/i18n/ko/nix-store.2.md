# nix-store.2

Nix 저장소를 조작하거나 쿼리.

## Usage
- `nix-store --gc` : 사용하지 않는 경로를 제거하는 등의 쓰레기 수집
- `nix-store --optimise` : 동일한 파일을 하드 링크로 연결하여 공간 사용량 줄이기
- `nix-store --delete /nix/store/...` : 특정 저장소 경로 삭제 (사용 중이지 않아야 함)
- `nix-store -q --tree /nix/store/...` : 저장소 경로(패키지)의 모든 의존성을 트리 형식으로 표시
- `du -cLsh $(nix-store -q --references /nix/store/...)` : 특정 저장소 경로와 모든 의존성의 총 크기 계산
- `nix-store -q --referrers /nix/store/...` : 특정 저장소 경로에 대한 모든 종속 항목 표시

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-store.html
